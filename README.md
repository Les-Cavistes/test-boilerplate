# Full-Stack Task Management Application

A modern web application built with Rust and SvelteKit for managing tasks.
Created for boilerplating our project.

## Tech Stack

### Backend

- Rust
- Rocket (Web framework)
- Diesel ORM
- SQLite Database

### Frontend

- SvelteKit 5.0
- TypeScript
- Biome (Formatting & Linting)

## Prerequisites

- Rust (latest stable version)
- Node.js (LTS version)
- SQLite3

## Project Structure

```
.
├── back/           # Rust backend
└── front/          # SvelteKit frontend
```

## Setup & Installation

### Backend Setup

1. Navigate to the backend directory:

```bash
cd back
```

2. Install Diesel CLI (if not already installed) (optional):

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

3. Set up the database (optional):

```bash
diesel setup
diesel migration run
```

4. Run the backend server:

```bash
cargo run
```

The backend will be available at `http://localhost:8000`

### Frontend Setup

1. Navigate to the frontend directory:

```bash
cd front
```

2. Install dependencies:

```bash
yarn install
```

3. Start the development server:

```bash
yarn run dev
```

The frontend will be available at `http://localhost:5173`

## Development

### Backend Development

- The backend uses Rocket's built-in auto-reload feature
- Database migrations are located in `back/migrations/`
- API endpoints are defined in `back/src/main.rs`
- Task model and operations are in `back/src/task.rs`

### Frontend Development

- Components and routes are in `front/src/routes/`
- TypeScript types are automatically generated in `.svelte-kit/`
- Use `npm run check` to type-check the project
- Biome handles formatting and linting

## Building for Production

### Backend

```bash
cd back
cargo build --release
```

### Frontend

```bash
cd front
yarn run build
```
