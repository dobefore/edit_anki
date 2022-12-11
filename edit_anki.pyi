from typing import Optional,List
class Collection:
    def __init__(self,col_path:Optional[str]) :
        '''
        col_path is the path of collection database,it looks like this:
        p=r'C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2'

        If this parameter is passed as None,in-memory database will be opened.

        When you want to open a collection in Anki folder,write like this:

        import edit_anki
        p=r'C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2'
        c=edit_anki.Collection(p)
        '''
    def get_deck_names(self, skip_empty_default: bool=False)-> List[str]:
        '''
        get all decks ,return their deck  names
        
        output,for example:
        [ 'dd',  'Default',  '成语小酷']
        '''

    def note_type_names(self)-> List[str]:
        '''
        get all note types , return their names    
        '''
    def all_notes_from_deck(self)->Optional[List[int,List[str]]]: 
        '''
        get all notes from a deck,return their note id and field values
        '''
    def add_note(self):
        '''
        create a new note from an existing notetype,deck.
        '''
    def set_note_field(self, note_id: int,deck_name: str,field_index: int,new_field: str):
        '''
        change contents of a field of a note

        we match whichever field we choose and if matched,change contents of whichever field we choose
        '''