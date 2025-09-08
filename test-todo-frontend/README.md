# todo_api

Generated Rust web API using [Rustফর্ম](https://github.com/your-org/rust-form)

## Getting Started

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Set up the database:
   ```bash
   cargo install sqlx-cli
   export DATABASE_URL=sqlite:todo_api.db
   sqlx database create
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## API Endpoints

### Todo (/todos)

- `POST /todos` - create
- `GET /todos` - read_all
- `GET /todos/:id` - read_one
- `PUT /todos/:id` - update
- `PATCH /todos/:id` - patch
- `DELETE /todos/:id` - delete

## Models

### Todo

Table: `todos`

| Field | Type | Constraints |
|-------|------|-------------|
| id | Option<i32> | Primary Key |
| title | String | Required |
| description | Option<String> | Optional |
| completed | Option<bool> | Optional |
| priority | Option<String> | Optional |
| due_date | Option<chrono::DateTime<chrono::Utc>> | Optional |
| created_at | Option<chrono::DateTime<chrono::Utc>> | Optional |
| updated_at | Option<chrono::DateTime<chrono::Utc>> | Optional |

