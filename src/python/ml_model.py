# import spacy

# def main():
#     nlp = spacy.load('en_core_web_sm')
#     doc = nlp("Rust is a systems programming language developed by Mozilla.")
#     print([(ent.text, ent.label_) for ent in doc.ents])

from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.linear_model import LogisticRegression
from sklearn.pipeline import make_pipeline

def main():
    model = make_pipeline(TfidfVectorizer(), LogisticRegression())
    #0 for positive, 1 for negtive
    model.fit(["I love Rust!", "I hate bugs."], [1, 0])
    print(model.predict(["Rust is amazing!"])) # 1
    print(model.predict(["I hate Rust!"])) # 0


if __name__ == "__main__":  
    main()