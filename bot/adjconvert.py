import re

def convert_to_ly_form(adjective_list):
    converted_words = {}
    
    for adj in adjective_list:
        adj = adj.lower().strip()
        
        if adj.endswith('le'):
            ly_form = adj[:-1] + 'y'
        elif adj.endswith('y') and len(adj) > 1 and adj[-2] not in 'aeiou':
            ly_form = adj[:-1] + 'ily'
        else:
            ly_form = adj + 'ly'

        converted_words[adj] = ly_form
    return converted_words

with open("adjectives") as f:
    converted_forms = convert_to_ly_form(f)
    for original, converted in converted_forms.items():
        with open("newadj", "a") as f:
            f.write(converted + "\n")

