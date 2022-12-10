//! It mainly focuses on Modifying existing decks and collections.
//! Functions It has:
//! 1. create a deck .And create a note from the deck and existing notetype.Export this deck.
//! 2. get all notes' fields from a specified deck.
//! 3. choose a deck and appoint one card,modifying one note of the card,for example,changing its tags or changing contents of one field,  and saving to db.   
 mod collection;
 mod error;
pub use anki::notetype::NoteField;
pub use anki::prelude::Notetype;
pub use collection::Collection;
pub use error::Result;
use pyo3::prelude::*;
// C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2
// C:\Users\Admin\AppData\Local\Programs\Python\Python310\Lib\site-packages

/// class help
#[pyclass(name = "Collection")]
struct CollectionW {
    pub col: Collection,
}

#[pymethods]
impl CollectionW {
    #[new]
    fn new(col_path: Option<String>) -> Result<Self> {
        let col = collection::Collection::new(col_path)?;
        Ok(CollectionW { col })
    }
    /// get all notes and show their fields
    pub fn get_all_notes(&mut self) -> Result<Vec<Vec<String>>> {
        let nt = self.col.get_all_notes();
        let mut v: Vec<Vec<String>> = Vec::new();
        for n in nt {
            let nts = n.fields().to_owned();
            v.push(nts);
        }
        Ok(v)
    }
    /// get all notes from a deck,return note id and fields of all notes
    pub fn all_notes_from_deck(
        &mut self,
        deck_name: &str,
    ) -> Result<Option<Vec<(i64, Vec<String>)>>> {
        let notes = self.col.all_notes_from_deck(deck_name)?;
        let mut note_vec = vec![];
        if let Some(nts) = notes {
            for nt in nts {
                let id = nt.id.0;
                note_vec.push((id, nt.fields().to_owned()));
            }
            Ok(Some(note_vec))
        } else {
            Ok(None)
        }
    }
    /// modify existing notes such as changing contents of fields
    fn set_note_field(
        &mut self,
        note_id: i64,
        deck_name: &str,
        field_index: usize,
        new_field: &str,
    ) -> Result<()> {
        let note = self.col.get_note_by_id(note_id)?;
        let deck = self.col.get_or_create_deck(deck_name)?;
        if let Some(nt) = note {
            self.col.set_note_field(nt, deck, field_index, new_field)?;
        }
        Ok(())
    }
    /// create a new note from an existing notetype,deck
    fn add_note(&mut self, fields: Vec<&str>, note_type_name: &str, deck_name: &str) -> Result<()> {
        let note = self.col.new_note(note_type_name, &fields)?;

        let deck = self.col.get_or_create_deck(deck_name)?;

        self.col.add_note(note, deck)?;

        Ok(())
    }
    /// get all note types , return their id and names    
    pub fn get_all_note_types(&mut self) -> Result<Vec<(i64, String)>> {
        let nts = self.col.get_all_note_types()?;
        Ok(nts)
    }
    /// return existing deck ids and deck names
    #[args(skip_empty_default = false)]
    #[pyo3(text_signature = "(&self, skip_empty_default: bool)")]
    pub fn get_all_deck_names(&self, skip_empty_default: bool) -> Result<Vec<(i64, String)>> {
        let d = self.col.get_all_deck_names(skip_empty_default);
        Ok(d?
            .iter()
            .map(|(a, b)| (a.to_owned(), b.to_string()))
            .collect())
    }
}
/// A Python module implemented in Rust.
#[pymodule]
fn edit_anki(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CollectionW>()?;
    Ok(())
}
