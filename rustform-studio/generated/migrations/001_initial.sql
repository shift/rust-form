CREATE TABLE configs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  yaml_content TEXT NOT NULL,
  is_template BOOLEAN NOT NULL DEFAULT 0,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);

CREATE  INDEX idx_configs_name ON configs (name);
CREATE  INDEX idx_configs_is_template ON configs (is_template);

CREATE TABLE components (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  uri TEXT NOT NULL,
  manifest_data TEXT NOT NULL,
  description TEXT NOT NULL,
  version TEXT NOT NULL,
  author TEXT NOT NULL,
  keywords TEXT NOT NULL,
  cached_at DATETIME NOT NULL
);

CREATE  INDEX idx_components_name ON components (name);
CREATE  INDEX idx_components_author ON components (author);

CREATE TABLE projects (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  config_id INTEGER NOT NULL,
  generated_at DATETIME NOT NULL,
  file_path TEXT NOT NULL,
  generation_log TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'String("pending")'
);

CREATE  INDEX idx_projects_config_id ON projects (config_id);
CREATE  INDEX idx_projects_status ON projects (status);

CREATE TABLE templates (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  category TEXT NOT NULL,
  description TEXT NOT NULL,
  yaml_content TEXT NOT NULL,
  tags TEXT NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT 1,
  created_at DATETIME NOT NULL
);

CREATE  INDEX idx_templates_category ON templates (category);
CREATE  INDEX idx_templates_is_public ON templates (is_public);

