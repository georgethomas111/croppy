# pip install tensorflow flask pillow

# app.py
from flask import Flask, request, jsonify
from PIL import Image
import tensorflow as tf
import numpy as np
import os

app = Flask(__name__)

# Load the MobileNet model
model = tf.keras.applications.MobileNetV2(weights="imagenet")

# Preprocess the image to the required size and scale
def preprocess_image(image):
    image = image.resize((224, 224))  # Resize to MobileNet's input size
    image_array = np.array(image)
    image_array = np.expand_dims(image_array, axis=0)  # Add batch dimension
    image_array = tf.keras.applications.mobilenet_v2.preprocess_input(image_array)  # Scale pixels
    return image_array

# Define a route to handle image classification
@app.route("/classify", methods=["POST"])
def classify_image():
    if "image" not in request.files:
        return jsonify({"error": "Please upload an image file"}), 400

    file = request.files["image"]
    image = Image.open(file).convert("RGB")  # Convert image to RGB

    # Preprocess and classify the image
    processed_image = preprocess_image(image)
    predictions = model.predict(processed_image)

    # Decode and retrieve the top 5 predictions
    decoded_predictions = tf.keras.applications.mobilenet_v2.decode_predictions(predictions, top=5)[0]
    results = [{"class_id": pred[0], "label": pred[1], "confidence": float(pred[2])} for pred in decoded_predictions]

    return jsonify({"predictions": results})

# Run the app
if __name__ == "__main__":
    app.run(debug=True)

