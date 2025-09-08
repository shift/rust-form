CREATE TABLE todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 0,
  priority TEXT NOT NULL DEFAULT 'String("medium")',
  due_date DATETIME,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);

CREATE  INDEX idx_todos_completed ON todos (completed);
CREATE  INDEX idx_todos_due_date ON todos (due_date);
CREATE  INDEX idx_todos_created_at ON todos (created_at);

