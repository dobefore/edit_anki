from typing import Optional,List,Tuple
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
    def get_all_deck_names(self, skip_empty_default: bool=False)-> List[Tuple[int,str]]:
        '''
        get all decks ,return their deck id and names
        
        output,for example:
        [(1666677001723, 'dd'), (1, 'Default'), (1664236622080, '成语小酷')]
        '''

    def get_all_note_types(self)-> List[Tuple[int,str]]:
        '''
        get all note types , return their id and names    
        '''