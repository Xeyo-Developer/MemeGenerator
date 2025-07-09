
<div align="center">
    <img src="/frontend/public/favicon.ico" width="256" alt="MemeGenerator Logo"/>
    <h1>MemeGenerator</h1>
</div>

A meme generator application with Rust backend and React frontend that allows users to generate random memes from a collection of templates.

## ✨ Features

- 🚀 **Generate random memes with a single click**
- 📥 **Download your favorite memes**
- 🔍 **View available meme templates**
- 🌐 **REST API backend built with Actix-web**
- 💅 **Modern UI with React and Tailwind CSS**

---

## Tech Stack 🛠️

### 🖼️ Frontend
- ReactJS
- Tailwind CSS
- Lucide React (icons)
- React Icons

### ⚙️ Backend
- Rust
- Actix-web framework
- Base64 encoding
- Random number generation

---

## Installation ⚙️

### Prerequisites
- Node.js (v16+ recommended)
- npm or yarn
- Rust (latest stable version)
- Cargo (Rust package manager)


### Backend Setup
```bash
# Navigate to the backend directory
cd backend
# Build
cargo build
# Run the server
cargo run
```

### Frontend Setup
```bash
# Navigate to the frontend directory
cd frontend
# Install dependencies
npm install
# Start the development server
npm start
```

--- 

## API Endpoints 🌍

| Method | Endpoint      | Description                                | Example Request / Response |
|--------|---------------|--------------------------------------------|-----------------------------|
| GET    | `/health`     | Check if the backend is running            | **Response:** `{ "status": "ok" }` |
| GET    | `/list`       | Get available meme templates               | **Response:** `["meme1.jpg", "meme2.jpg", ...]` |
| POST   | `/generate`   | Generate a meme from template & text       | **Request:** `{ "template": "meme1.jpg", "top_text": "Hello", "bottom_text": "World" }`<br>**Response:** `{ "image_data": "base64-encoded-image-string" }` |
