// determine where examples folder should be,see following link for package layout
// https://doc.rust-lang.org/cargo/guide/project-layout.html
#![allow(dead_code)]
use edit_anki::{Collection, Result, NoteField, Notetype};
/// create a in-memory collection and create a default note type with note fields `Front` and `Back`
fn open_collection() -> Result<(Collection, Notetype)> {
    // create colletion,deck and note;
    let mut col = Collection::new(None)?;
    let mut notetype = Notetype::default();
    notetype.fields = vec![NoteField::new("Front"), NoteField::new("Back")];
    notetype.name = "name".into();
    col.add_note_type(&mut notetype)?;
    Ok((col, notetype))
}
/// create a in-memory collection,create a deck and add a note to it.
fn add_note() -> Result<()> {
    // create colletion,deck and note;
    let (mut col, mut notetype) = open_collection()?;

    let note = col.new_note(&notetype.name, &["front", "back"])?;

    let deck = col.get_or_create_deck("deck")?;

    col.add_note_type(&mut notetype)?;
    col.add_note(note, deck)?;

    println!("{:?}", col.get_all_notes());
    Ok(())
}
/// modify one exist note,change its first field content
fn modify_note() -> Result<()> {
    // open collection and add one note anyway even though that note already existed.
    let col_path = r"C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2";
    let mut col = Collection::new(Some(col_path.to_string()))?;
    let deck = col.get_or_create_deck("new_deck")?;
    let note = col.new_note("Basic", &["front", "back"])?;
    col.add_note(note, deck.clone())?;

    let notes = col.all_notes_from_deck("new_deck")?;
    // loop through all notes to find our desired notes,and,change their field contents.
    if let Some(nt) = notes {
        for note in nt {
            let fields = note.fields();
            if fields.first().unwrap() == "front" {
                let note = note;

                col.set_note_field(note, deck.clone(), 0, "after modifying")?;
            }
        }
    }

    Ok(())
}

fn remove_notes() -> Result<()> {
    // remove notes whose contents of first field contain words "front"
    // open collection and add one note anyway even though that note already existed.
    let col_path = r"C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2";
    let mut col = Collection::new(Some(col_path.to_string()))?;
    let deck = col.get_or_create_deck("new_deck")?;
    let note = col.new_note("Basic", &["front", "back"])?;
    col.add_note(note, deck)?;
    let mut notes_to_remove = vec![];
    let notes = col.all_notes_from_deck("new_deck")?;
    // loop through all notes to find our desired notes,and,filter them out .
    if let Some(nt) = notes {
        for note in nt {
            let fields = note.fields();
            if fields.first().unwrap() == "front" {
                let note = note;

                notes_to_remove.push(note);
            }
        }

        // remove notes by its ids.
        let note_ids = notes_to_remove.iter().map(|n| n.id).collect::<Vec<_>>();
        col.remove_notes(&note_ids)?;
    }

    Ok(())
}
fn main() {
    add_note().unwrap();
}
