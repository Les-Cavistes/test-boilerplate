# Task Management Application

A modern, type-safe task management application built with SvelteKit. Features real-time task updates, optimistic UI
updates, and a clean, responsive design.

## Features

- ✨ Create, toggle, and delete tasks
- 🔄 Real-time UI updates
- 🎨 Clean, responsive design
- 🔒 Type-safe implementation
- ⚡ Optimistic updates for better UX

## Technical Stack

- **Frontend Framework**: SvelteKit
- **Language**: TypeScript
- **Styling**: SCSS
- **API Communication**: Axios
- **Validation**: Zod
- **State Management**: Svelte Stores

## Project Structure

```
src/
├── lib/
│   ├── stores/      # Svelte stores for state management
│   ├── styles/      # SCSS styles
│   └── types/       # TypeScript type definitions
└── routes/
    └── api/         # API route handlers
```

## Getting Started

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Install dependencies:
```bash
npm install
# or
pnpm install
# or
yarn
```

3. Set up environment variables:
   Create a `.env` file with:

```
PUBLIC_BACK_ENDPOINT=<your-backend-url>
```

4. Start development server:
```bash
npm run dev
# or
npm run dev -- --open
```

## Development

### API Routes

The application uses SvelteKit's server routes for API handling. API endpoints are located in `src/routes/api/` and
include:

- `GET /api/task/all` - Fetch all tasks
- `POST /api/task` - Create new task
- `PATCH /api/task/[id]/toggle` - Toggle task completion
- `DELETE /api/task/[id]` - Delete task

### State Management

Task state is managed through a custom Svelte store (`src/lib/stores/tasks.ts`) that provides:

- Sorted task list management
- Optimistic updates
- Type-safe operations

### Building

To create a production version:

```bash
npm run build
```

Preview the production build with:

```bash
npm run preview
```

## Deployment

Before deploying, ensure you:

1. Set up environment variables
2. Install appropriate [adapter](https://svelte.dev/docs/kit/adapters) for your target environment
3. Build the application
