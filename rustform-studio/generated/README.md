# rustform_studio

Generated Rust web API using [Rustফর্ম](https://github.com/your-org/rust-form)

## Getting Started

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Set up the database:
   ```bash
   cargo install sqlx-cli
   export DATABASE_URL=sqlite:rustform_studio.db
   sqlx database create
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## API Endpoints

### Config (/configs)

- `POST /configs` - create
- `GET /configs` - read_all
- `GET /configs/:id` - read_one
- `PUT /configs/:id` - update
- `DELETE /configs/:id` - delete

### Component (/components)

- `POST /components` - create
- `GET /components` - read_all
- `GET /components/:id` - read_one
- `PUT /components/:id` - update
- `DELETE /components/:id` - delete

### Project (/projects)

- `POST /projects` - create
- `GET /projects` - read_all
- `GET /projects/:id` - read_one
- `PUT /projects/:id` - update
- `DELETE /projects/:id` - delete

### Template (/templates)

- `POST /templates` - create
- `GET /templates` - read_all
- `GET /templates/:id` - read_one
- `PUT /templates/:id` - update
- `DELETE /templates/:id` - delete

## Models

### Config

Table: `configs`

| Field | Type | Constraints |
|-------|------|-------------|
| id | Option<i32> | Primary Key |
| name | String | Required |
| description | Option<String> | Optional |
| yaml_content | String | Required |
| is_template | Option<bool> | Optional |
| created_at | Option<chrono::DateTime<chrono::Utc>> | Optional |
| updated_at | Option<chrono::DateTime<chrono::Utc>> | Optional |

### Component

Table: `components`

| Field | Type | Constraints |
|-------|------|-------------|
| id | Option<i32> | Primary Key |
| name | String | Required |
| uri | String | Required |
| manifest_data | Option<serde_json::Value> | Optional |
| description | Option<String> | Optional |
| version | Option<String> | Optional |
| author | Option<String> | Optional |
| keywords | Option<serde_json::Value> | Optional |
| cached_at | Option<chrono::DateTime<chrono::Utc>> | Optional |

### Project

Table: `projects`

| Field | Type | Constraints |
|-------|------|-------------|
| id | Option<i32> | Primary Key |
| name | String | Required |
| config_id | i32 | Required |
| generated_at | Option<chrono::DateTime<chrono::Utc>> | Optional |
| file_path | Option<String> | Optional |
| generation_log | Option<String> | Optional |
| status | Option<String> | Optional |

### Template

Table: `templates`

| Field | Type | Constraints |
|-------|------|-------------|
| id | Option<i32> | Primary Key |
| name | String | Required |
| category | String | Required |
| description | Option<String> | Optional |
| yaml_content | String | Required |
| tags | Option<serde_json::Value> | Optional |
| is_public | Option<bool> | Optional |
| created_at | Option<chrono::DateTime<chrono::Utc>> | Optional |

