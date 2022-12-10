import edit_anki
p=r'C:\Users\Admin\AppData\Roaming\Anki2\Android\collection.anki2'
c=edit_anki.Collection(p)
d=c.get_all_deck_names()
print(d)
print(c.all_notes_from_deck("new_deck"))
