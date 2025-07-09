<div align="center">
    <h1>MemeGenerator</h1>
</div>

A meme generator application with Rust backend and React frontend that allows users to browse, generate, and manage memes from a collection of templates.

## ✨ Features

- 🎲 **Generate random memes with a single click**
- 📥 **Download your favorite memes**
- 🔍 **Search through meme templates**
- ⭐ **Favorite system to save your preferred memes**
- 📊 **View statistics about your meme collection**
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
- Chrono for timestamps
- Serde for JSON serialization

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

# Build the project
cargo build

# Run the server
cargo run
```

The backend server will start on `http://localhost:8080`

### Frontend Setup
```bash
# Navigate to the frontend directory
cd frontend

# Install dependencies
npm install

# Start the development server
npm start
```

The frontend will start on `http://localhost:3000`

---

## API Endpoints 🌍

| Method | Endpoint                | Description                                | Example Response |
|--------|-------------------------|--------------------------------------------|--------------------|
| GET    | `/health`              | Check if the backend is running            | `{ "status": "OK", "message": "Meme server is running properly", "timestamp": "2024-01-01T12:00:00Z" }` |
| GET    | `/list`                | Get all available meme templates           | `{ "templates": [{"name": "meme1.jpg", "size_bytes": 12345, "file_type": "jpg", ...}], "total_count": 50 }` |
| GET    | `/generate`            | Generate a single random meme             | `{ "template_name": "meme1.jpg", "image_url": "data:image/jpeg;base64,...", "content_type": "image/jpeg", "size_bytes": 12345, "generated_at": "2024-01-01T12:00:00Z" }` |
| GET    | `/meme/{filename}`     | Get a specific meme by filename            | `{ "template_name": "meme1.jpg", "image_url": "data:image/jpeg;base64,...", "content_type": "image/jpeg", "size_bytes": 12345, "requested_at": "2024-01-01T12:00:00Z" }` |
| GET    | `/random/{count}`      | Generate multiple random memes (max 50)   | `{ "memes": [...], "count": 5, "generated_at": "2024-01-01T12:00:00Z" }` |
| GET    | `/stats`               | Get statistics about meme collection       | `{ "total_memes": 100, "total_size_bytes": 5000000, "average_file_size": 50000, "largest_file_name": "big_meme.jpg", "file_types": {"jpeg": 60, "png": 30, "gif": 10} }` |
| GET    | `/search?q=query`      | Search for memes by filename              | `{ "query": "cat", "results": [...], "count": 5 }` |
| POST   | `/favorite`            | Toggle favorite status for a meme         | **Request:** `{ "meme_name": "meme1.jpg" }`<br>**Response:** `{ "meme_name": "meme1.jpg", "is_favorite": true, "message": "Added to favorites" }` |
| GET    | `/favorites`           | Get list of favorite memes                | `{ "favorites": ["meme1.jpg", "meme2.jpg"], "count": 2 }` |
