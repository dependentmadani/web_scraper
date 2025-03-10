import pandas as pd
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.linear_model import LogisticRegression
from sklearn.pipeline import make_pipeline
import joblib

# Sample training data (modify this with your dataset)
data = {
    "text": [
        "I love Rust!",
        "Rust is amazing!",
        "I hate bugs.",
        "This code is terrible.",
        "Rust is a programming language.",
        "The documentation is helpful.",
    ],
    "sentiment": [
        "positive", "positive", "negative", "negative", "neutral", "positive"
    ]
}

# Create a DataFrame
df = pd.DataFrame(data)

# Split into features (X) and labels (y)
X = df["text"]
y = df["sentiment"]

# Create a pipeline: TF-IDF vectorizer + logistic regression
model = make_pipeline(
    TfidfVectorizer(),
    LogisticRegression()
)

# Train the model
model.fit(X, y)

# Save the model to a file
joblib.dump(model, "sentiment_model.pkl")
print("Model saved as sentiment_model.pkl")