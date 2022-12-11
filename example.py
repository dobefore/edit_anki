import edit_anki
col_path=r'C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2'
def modify_note():
    '''
    modify field contents of a note
    Following example shows that we will match the first field value
    and if matched, change the contents of the second field of the note.
    '''
    col=edit_anki.Collection(col_path)
    notes_from_deck=col.all_notes_from_deck("deck_name")
    for note_id,fields in notes_from_deck:
        # match the first field value to modify the field content
        if fields[0]=="word":
            col.set_note_field(note_id,"deck_name",1,"new_field_value")


def add_new_note(args):
    '''
    add a note to an existing deck.
    Following example shows that will add a new note to an existing deck named `deck`,
    and its note type name is `Basic`,its field values are "1","2","3",respectively.
    '''
    col=edit_anki.Collection(col_path)
    col.add_note(["1","2","3"],"Basic","deck")

def get_notes_from_deck():
    '''
    get all notes from an existing deck.

    its return type will be a list of tuple, 
    each tuple contains the note id and its field values.
    '''
    col=edit_anki.Collection(col_path)
    notes_from_deck=col.all_notes_from_deck("deck_name")
def get_note_type_names():
    '''
    get names of all note types.
    '''
    col=edit_anki.Collection(col_path)
    note_type_names=col.note_type_names()

def get_deck_names():
    '''
    get names of all decks.
    '''
    col=edit_anki.Collection(col_path)
    deck_names=col.get_deck_names()

l=[1,2]
print(l[0])