# Getting Started

This comprehensive guide will walk you through creating your first full-stack application with Rust-form, from installation to deployment.

## 🎯 What You'll Build

By the end of this tutorial, you'll have a complete todo application with:
- **Rust backend** with CRUD API endpoints
- **React frontend** with forms and data tables
- **Perfect type safety** between frontend and backend
- **Database integration** with SQLite

## 📋 Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Node.js 18+** - For frontend development
- **Git** - For version control

**Optional but Recommended:**
- **Nix** - For reproducible development environment

## 📦 Installation

### Option 1: Install from Source (Recommended)

```bash
git clone https://github.com/rust-form/rust-form.git
cd rust-form
cargo install --path rustform-cli

# Verify installation
rustform --version
```

### Option 2: Using Nix (Development Environment)

```bash
git clone https://github.com/rust-form/rust-form.git
cd rust-form

# Enter development shell with all dependencies
nix develop

# Install locally
cargo install --path rustform-cli
```

## 🚀 Your First Application

### Step 1: Create a Configuration File

Create a new file called `todo.yml`:

```yaml
# todo.yml - Complete todo application configuration
project:
  name: "todo_app"
  description: "A simple but powerful todo application"

database:
  type: "sqlite"
  url_env: "DATABASE_URL"

server:
  port: 8080
  cors:
    origins: ["http://localhost:3000"]

models:
  Todo:
    fields:
      id:
        type: "uuid"
        primary_key: true
        default: "gen_random_uuid()"
      title:
        type: "string"
        validation:
          min_length: 1
          max_length: 200
          message: "Title must be 1-200 characters"
      description:
        type: "text"
        optional: true
        validation:
          max_length: 1000
      completed:
        type: "boolean"
        default: false
      priority:
        type: "enum"
        values: ["low", "medium", "high"]
        default: "medium"
      due_date:
        type: "date"
        optional: true
      created_at:
        type: "datetime"
        default: "now()"
      updated_at:
        type: "datetime"
        auto_update: true

  Category:
    fields:
      id:
        type: "uuid"
        primary_key: true
        default: "gen_random_uuid()"
      name:
        type: "string"
        validation:
          min_length: 1
          max_length: 100
      color:
        type: "string"
        optional: true
        validation:
          pattern: "^#[0-9A-Fa-f]{6}$"
          message: "Must be a valid hex color"
      created_at:
        type: "datetime"
        default: "now()"

frontend:
  target: "react"
  typescript_output_dir: "../frontend/src/generated"
  generate_ui_for: ["Todo", "Category"]
  
  components:
    Todo:
      form_fields: ["title", "description", "priority", "due_date"]
      list_columns: ["title", "priority", "due_date", "completed"]
      features: ["create", "edit", "delete", "search"]
      
    Category:
      form_fields: ["name", "color"]
      list_columns: ["name", "color", "created_at"]
      features: ["create", "edit", "delete"]

  framework_config:
    react:
      styling: "tailwind"
      state_management: "react-query"
      forms: "react-hook-form"
      validation: "zod"
```

### Step 2: Generate Your Application

```bash
# Generate the full-stack application
rustform generate todo.yml

# This creates:
# todo_app/
# ├── backend/     # Rust API server
# └── frontend/    # React application
```

### Step 3: Set Up the Backend

```bash
cd todo_app/backend

# Create environment file
echo "DATABASE_URL=sqlite:todo.db" > .env

# Run database migrations and start server
cargo run

# Your API is now running on http://localhost:8080! 🎉
```

**You should see:**
```
2024-01-15T10:30:00.123Z INFO todo_app: Database migrations completed
2024-01-15T10:30:00.125Z INFO todo_app: listening on 127.0.0.1:8080
```

### Step 4: Set Up the Frontend

Open a new terminal:

```bash
cd todo_app/frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Your frontend is now running on http://localhost:3000! 🎉
```

### Step 5: Test Your Application

Visit `http://localhost:3000` in your browser. You should see:

- **Todo List** with create, edit, delete functionality
- **Category Management** for organizing todos
- **Type-safe forms** with validation
- **Real-time updates** between frontend and backend

**Try these actions:**
1. Create a new todo with title "Learn Rust-form"
2. Set priority to "high" and add a due date
3. Create a category called "Learning" with color #3B82F6
4. Edit the todo to mark it as completed
5. Use the search functionality

## 🔍 Understanding What Was Generated

### Backend Structure

```
backend/
├── src/
│   ├── main.rs          # Axum server setup
│   ├── models.rs        # Todo and Category structs
│   ├── handlers.rs      # CRUD API endpoints
│   ├── database.rs      # SQLx database setup
│   └── error.rs         # Error handling
├── migrations/          # Database migrations
├── Cargo.toml           # Dependencies
└── .env                 # Environment variables
```

**Key Features Generated:**
- **REST API Endpoints**: GET, POST, PUT, DELETE for each model
- **Database Integration**: SQLx with compile-time verified queries
- **Type Safety**: All models use Rust's type system
- **Error Handling**: Comprehensive error responses
- **CORS Support**: Configured for frontend integration

### Frontend Structure

```
frontend/
├── src/
│   ├── generated/
│   │   ├── types/       # Auto-generated TypeScript interfaces
│   │   └── components/  # Auto-generated React components
│   │       ├── Todo/
│   │       │   ├── TodoForm.tsx
│   │       │   ├── TodoList.tsx
│   │       │   └── index.ts
│   │       └── Category/
│   ├── pages/           # Page components
│   ├── hooks/           # Custom React hooks
│   └── App.tsx          # Main application
├── package.json         # Dependencies
└── tailwind.config.js   # Styling configuration
```

**Key Features Generated:**
- **Type-Safe Components**: Forms and lists with full TypeScript support
- **React Query Integration**: Automatic caching and synchronization
- **Form Validation**: Zod schemas generated from backend validation
- **Responsive Design**: Tailwind CSS for modern styling

### Type Safety in Action

**Backend (Rust):**
```rust
// Generated in models.rs
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../frontend/src/generated/types/")]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Priority,
    pub due_date: Option<chrono::NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

**Frontend (TypeScript):**
```typescript
// Auto-generated in types/models.ts
export interface Todo {
  id: string;
  title: string;
  description?: string;
  completed: boolean;
  priority: "low" | "medium" | "high";
  due_date?: string;
  created_at: string;
  updated_at: string;
}
```

**React Component:**
```tsx
// Generated TodoForm.tsx uses the interface
import { Todo, CreateTodoRequest } from '../types/models';

interface TodoFormProps {
  todo?: Todo;
  onSubmit: (data: CreateTodoRequest) => void;
}

export function TodoForm({ todo, onSubmit }: TodoFormProps) {
  // TypeScript knows exactly what fields are available!
  const { register, handleSubmit } = useForm<CreateTodoRequest>();
  // ...
}
```

## 🎨 Customizing Your Application

### Adding Validation

Edit your `todo.yml` to add more validation rules:

```yaml
models:
  Todo:
    fields:
      title:
        type: "string"
        validation:
          min_length: 1
          max_length: 200
          pattern: "^[a-zA-Z0-9\\s\\-_]+$"
          message: "Title can only contain letters, numbers, spaces, hyphens, and underscores"
      
      email:
        type: "string"
        optional: true
        validation:
          format: "email"
          message: "Must be a valid email address"
```

Regenerate to update both backend validation and frontend form validation:

```bash
rustform generate todo.yml
```

### Switching Frontend Frameworks

Want to try Vue instead of React? Just change one line:

```yaml
frontend:
  target: "vue"  # Changed from "react"
  # ... rest of config stays the same
```

```bash
rustform generate todo.yml
cd todo_app/frontend
npm install && npm run dev
```

Your Vue application will have the same functionality with Vue-specific patterns!

### Adding New Models

Add a new model to your configuration:

```yaml
models:
  # ... existing models
  
  User:
    fields:
      id: { type: "uuid", primary_key: true }
      username: { type: "string", unique: true }
      email: { type: "string", validation: { format: "email" } }
      created_at: { type: "datetime", default: "now()" }

frontend:
  generate_ui_for: ["Todo", "Category", "User"]  # Add User
```

Regenerate and you'll have complete CRUD functionality for users!

## 🚀 Next Steps

### 1. Explore Advanced Features

- **Relationships**: Add foreign keys between models
- **Authentication**: Add user authentication system
- **File Uploads**: Handle image and file uploads
- **Real-time**: Add WebSocket support for live updates

### 2. Deployment

#### Backend Deployment
```bash
cd backend
cargo build --release

# The binary is ready for deployment
./target/release/todo_app
```

#### Frontend Deployment
```bash
cd frontend
npm run build

# Deploy the dist/ folder to your hosting provider
```

### 3. Learn More

- **[Configuration Reference](CONFIG_REFERENCE.md)** - Complete YAML schema
- **[Frontend Generation](FRONTEND_GENERATION.md)** - Multi-framework support
- **[Templates](TEMPLATES.md)** - Customize code generation
- **[Architecture](ARCHITECTURE.md)** - Technical deep dive

### 4. Join the Community

- **GitHub**: [rust-form/rust-form](https://github.com/rust-form/rust-form)
- **Discord**: [Join our Discord](https://discord.gg/rust-form)
- **Twitter**: [@rustform](https://twitter.com/rustform)

## 🐛 Troubleshooting

### Common Issues

**1. Compilation Errors**
```bash
# Ensure you have the latest Rust
rustup update

# Clear cargo cache
cargo clean && cargo build
```

**2. Database Connection Issues**
```bash
# Verify your .env file
cat backend/.env

# Should contain:
# DATABASE_URL=sqlite:todo.db
```

**3. Frontend Type Errors**
```bash
# Regenerate TypeScript types
cd backend && cargo check
cd ../frontend && npm run type-check
```

**4. Port Conflicts**
```yaml
# Change ports in your config
server:
  port: 8081  # Use different port

frontend:
  dev_server:
    port: 3001  # Use different port
```

### Getting Help

If you encounter issues:

1. Check the [troubleshooting guide](docs/TROUBLESHOOTING.md)
2. Search [existing issues](https://github.com/rust-form/rust-form/issues)
3. Ask on [Discord](https://discord.gg/rust-form)
4. Create a [new issue](https://github.com/rust-form/rust-form/issues/new)

## ✨ What's Next?

You've successfully created your first full-stack application with Rust-form! This is just the beginning. With the foundation you've built, you can:

- Add complex business logic
- Integrate with external APIs
- Build mobile apps using the same backend
- Scale to handle thousands of users
- Deploy to production with confidence

**Happy coding with Rust-form!** 🦀✨