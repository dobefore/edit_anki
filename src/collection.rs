// from anki/rslib/src/tests.rs
use crate::error::{EditAnkiError, Result};
use anki::collection::CollectionBuilder;
use anki::prelude::{Collection as CollectionW, Deck, Note, NoteId, Notetype};
use anki::search::SortMode;
pub struct Collection {
    pub col: CollectionW,
}

// get all notes
fn open_collection(col_path: Option<String>) -> Result<Collection> {
    let col = match col_path {
        Some(p) => CollectionBuilder::new(p).build(),
        None => CollectionBuilder::default().build(),
    };
    Ok(Collection { col: col? })
}

impl Collection {
    pub fn get_all_notes(&mut self) -> Vec<Note> {
        self.col.storage.get_all_notes()
    }
    /// create and return a new empty deck if there is no such a deck,else return an existing deck.   
    pub fn get_or_create_deck(&mut self, name: &str) -> Result<Deck> {
        let d = self.col.get_or_create_normal_deck(name)?;
        Ok(d)
    }

    pub fn get_field_names_from_notetype(
        &mut self,
        note_type_name: &str,
    ) -> Result<Option<Vec<String>>> {
        let note_type = self.col.get_notetype_by_name(note_type_name)?;
        if let Some(notetype) = note_type {
            let field = &notetype.fields;
            Ok(Some(
                field.iter().map(|e| e.name.to_string()).collect::<Vec<_>>(),
            ))
        } else {
            Ok(None)
        }
    }
    pub fn get_note_by_id(&mut self, note_id: i64) -> Result<Option<Note>> {
        let note = self.col.storage.get_note(note_id.into())?;
        Ok(note)
    }
    pub fn all_notes_from_deck(&mut self, deck_name: &str) -> Result<Option<Vec<Note>>> {
        // First,get deck by name.
        let mut notes = vec![];
        let deck = self.get_or_create_deck(deck_name)?;
        let cards = self.col.storage.all_cards_in_single_deck(deck.id)?;
        for cid in cards {
            let card = self.col.storage.get_card(cid)?;
            if let Some(c) = card {
                let nid = c.note_id;
                let note = self.col.storage.get_note(nid)?;
                // I think here it's safe to unwrap,as there exists a card,then there exists a note,isn't it?
                notes.push(note.unwrap());
            }
        }
        if notes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(notes))
        }
    }
    pub fn add_note_type(&mut self, note_type: &mut Notetype) -> Result<()> {
        self.col.add_notetype(note_type, true)?;
        Ok(())
    }
    ///  create a new note from a specified note type.Thus fields are also specified in that
    /// note type.
    ///
    /// # example
    /// Assuming we have a note type named `basic` ,whose fields are `front` and `basic`.
    ///
    /// Additional operation have to be performed,that is `col.add_note()`
    pub fn new_note(&mut self, note_type: &str, fields: &[&str]) -> Result<Note> {
        match self.col.get_notetype_by_name(note_type)? {
            Some(note_type) => {
                let mut note = note_type.new_note();
                for (idx, v) in fields.iter().enumerate() {
                    note.set_field(idx, v.to_string())?;
                }

                Ok(note)
            }
            None => Err(EditAnkiError::NoteType(
                "note type not found while adding new note".into(),
            )),
        }
    }
    ///get fields of a [`Note`]
    pub fn fields_of_note(note: &Note) -> &Vec<String> {
        note.fields()
    }
    /// open or create collection database
    ///
    /// open collection in memory if col_path is `None`
    pub fn new(col_path: Option<String>) -> Result<Self> {
        let col = open_collection(col_path);
        match col {
            Ok(col) => Ok(col),
            Err(e) => Err(e),
        }
    }
    pub fn remove_notes(&mut self, note_ids: &[NoteId]) -> Result<()> {
        self.col.remove_notes(note_ids)?;
        Ok(())
    }
    pub fn set_note_field(
        &mut self,
        mut note: Note,
        deck: Deck,
        field_index: usize,
        new_field: &str,
    ) -> Result<()> {
        // this will empty sort field and checksum,so better remove existing notes
        // and create a new copy of it.
        note.set_field(field_index, new_field)?;
        self.col.remove_notes(&[note.id])?;
        // set it to 0,wait fot it to be re-allocated an id.
        note.id = 0.into();
        self.add_note(note.clone(), deck)?;

        Ok(())
    }
    pub fn add_note(&mut self, mut note: Note, deck: Deck) -> Result<()> {
        self.col.add_note(&mut note, deck.id)?;
        Ok(())
    }
    pub fn get_all_note_types(&mut self) -> Result<Vec<(i64, String)>> {
        let nts = self.col.get_all_notetypes()?;

        let nt = nts
            .iter()
            .map(|(nid, nt)| (nid.0, nt.name.to_string()))
            .collect::<Vec<_>>();
        Ok(nt)
    }
    /// get and print notetype if there exists such a type by name

    pub fn get_all_deck_names(&self, skip_empty_default: bool) -> Result<Vec<(i64, String)>> {
        let d = self.col.get_all_deck_names(skip_empty_default);
        Ok(d?.iter().map(|(a, b)| (a.0, b.to_string())).collect())
    }

    pub fn search_notes(&mut self, search: &str, mode: SortMode) -> Result<Vec<NoteId>> {
        Ok(self.col.search_notes(search, mode)?)
    }
}
