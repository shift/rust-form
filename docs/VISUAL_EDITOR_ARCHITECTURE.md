# Rust-form Visual Config Editor - Architecture Design

## Overview

A web-based visual editor for creating Rust-form YAML configurations through drag-and-drop interface. This project serves as dogfooding for Rust-form itself, being built entirely with Rust-form-generated backend and frontend.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React)                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Component     │  │   Config        │  │   Preview &     │ │
│  │   Palette       │  │   Builder       │  │   Export        │ │
│  │                 │  │                 │  │                 │ │
│  │ • Models        │  │ • Visual Form   │  │ • YAML View     │ │
│  │ • Endpoints     │  │ • Drag & Drop   │  │ • Validation    │ │
│  │ • Middleware    │  │ • Property Edit │  │ • Generation    │ │
│  │ • Components    │  │ • Relationships │  │ • Download      │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                     API Client Layer                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Config API    │  │  Component API  │  │  Validation API │ │
│  │                 │  │                 │  │                 │ │
│  │ • CRUD configs  │  │ • Discover      │  │ • Schema check  │ │
│  │ • Templates     │  │ • Install       │  │ • Live feedback │ │
│  │ • Projects      │  │ • Catalog       │  │ • Error display │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                │
                                │ HTTP/REST
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Backend (Rust-form Generated)                │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Config        │  │   Component     │  │   Project       │ │
│  │   Management    │  │   Discovery     │  │   Generation    │ │
│  │                 │  │                 │  │                 │ │
│  │ • Store configs │  │ • Scan registry │  │ • Run rustform  │ │
│  │ • Templates     │  │ • Local cache   │  │ • File management│ │
│  │ • Validation    │  │ • Metadata      │  │ • Export/import │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                      Database (SQLite)                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Configs       │  │   Components    │  │   Projects      │ │
│  │                 │  │                 │  │                 │ │
│  │ • id            │  │ • id            │  │ • id            │ │
│  │ • name          │  │ • name          │  │ • name          │ │
│  │ • yaml_content  │  │ • uri           │  │ • config_id     │ │
│  │ • created_at    │  │ • manifest      │  │ • generated_at  │ │
│  │ • updated_at    │  │ • cached_at     │  │ • file_path     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Data Models

### Core Data Models

```yaml
# Configuration Management
Config:
  table_name: configs
  fields:
    id:
      type: integer
      primary_key: true
      auto_increment: true
    name:
      type: string
      required: true
      max_length: 255
    description:
      type: text
    yaml_content:
      type: text
      required: true
    is_template:
      type: boolean
      default: false
    created_at:
      type: datetime
      auto_now_add: true
    updated_at:
      type: datetime
      auto_now: true

# Component Registry
Component:
  table_name: components
  fields:
    id:
      type: integer
      primary_key: true
      auto_increment: true
    name:
      type: string
      required: true
      unique: true
    uri:
      type: string
      required: true
    manifest_data:
      type: json
    description:
      type: text
    version:
      type: string
    author:
      type: string
    keywords:
      type: json
    cached_at:
      type: datetime
      auto_now: true

# Generated Projects
Project:
  table_name: projects
  fields:
    id:
      type: integer
      primary_key: true
      auto_increment: true
    name:
      type: string
      required: true
    config_id:
      type: integer
      required: true
    generated_at:
      type: datetime
      auto_now_add: true
    file_path:
      type: string
    generation_log:
      type: text
    status:
      type: string
      default: "pending"
  relationships:
    config:
      type: many_to_one
      model: Config
      foreign_key: config_id

# Template Library
Template:
  table_name: templates
  fields:
    id:
      type: integer
      primary_key: true
      auto_increment: true
    name:
      type: string
      required: true
    category:
      type: string
      required: true
    description:
      type: text
    yaml_content:
      type: text
      required: true
    tags:
      type: json
    is_public:
      type: boolean
      default: true
    created_at:
      type: datetime
      auto_now_add: true
```

## API Endpoints

```yaml
# Configuration Management
/configs:
  - GET: List all configurations
  - POST: Create new configuration
/configs/{id}:
  - GET: Get specific configuration
  - PUT: Update configuration
  - DELETE: Delete configuration

# Component Discovery
/components:
  - GET: List available components
  - POST: Add/refresh component
/components/search:
  - GET: Search components by keyword/tag
/components/{name}:
  - GET: Get component details
  - POST: Install component

# Project Generation
/projects:
  - GET: List generated projects
  - POST: Generate new project
/projects/{id}:
  - GET: Get project details
  - DELETE: Remove project files

# Validation & Preview
/validate:
  - POST: Validate YAML configuration
/preview:
  - POST: Preview generated code

# Templates
/templates:
  - GET: List configuration templates
  - POST: Create template
/templates/{id}:
  - GET: Get template
  - PUT: Update template
  - DELETE: Delete template
```

## Frontend Component Architecture

### 1. Component Palette (Left Sidebar)

```tsx
interface ComponentPaletteProps {
  onDragStart: (item: PaletteItem) => void;
  searchQuery: string;
  selectedCategory: string;
}

interface PaletteItem {
  type: 'model' | 'endpoint' | 'middleware' | 'component';
  name: string;
  icon: string;
  template: object;
  description: string;
}
```

**Features:**
- Categorized component library
- Search and filter functionality
- Drag-and-drop initiation
- Template previews
- Component documentation

### 2. Visual Config Builder (Center)

```tsx
interface ConfigBuilderProps {
  config: ConfigState;
  onConfigChange: (config: ConfigState) => void;
  onDrop: (item: PaletteItem, position: DropPosition) => void;
}

interface ConfigState {
  projectInfo: ProjectInfo;
  database: DatabaseConfig;
  models: ModelConfig[];
  endpoints: EndpointConfig[];
  middleware: MiddlewareConfig[];
  frontend?: FrontendConfig;
  components: ComponentMap;
}
```

**Features:**
- Visual form builder
- Drag-and-drop zones
- Property editors
- Relationship visualizer
- Real-time validation

### 3. Preview & Export (Right Sidebar)

```tsx
interface PreviewPanelProps {
  config: ConfigState;
  validationErrors: ValidationError[];
  onExport: (format: 'yaml' | 'json') => void;
  onGenerate: () => void;
}
```

**Features:**
- Live YAML preview
- Validation error display
- Export functionality
- Generation controls
- File download

## Drag & Drop System

### Drop Zones

1. **Model Zone**: Add new data models
2. **Endpoint Zone**: Add API endpoints
3. **Middleware Zone**: Add middleware components
4. **Frontend Zone**: Configure frontend settings
5. **Component Zone**: Add component dependencies

### Drag Operations

```tsx
interface DragData {
  type: string;
  payload: any;
  sourceId?: string;
}

interface DropHandler {
  onDrop: (data: DragData, targetZone: string) => boolean;
  canDrop: (data: DragData, targetZone: string) => boolean;
}
```

## Real-time Validation

### Client-side Validation
- Schema validation using JSON Schema
- Field requirement checks
- Type validation
- Cross-reference validation

### Server-side Validation
- Rust-form config parser validation
- Component availability checks
- Dependency resolution
- Template compilation

## Technology Stack

### Backend (Generated by Rust-form)
- **Framework**: Axum
- **Database**: SQLite
- **ORM**: SQLx
- **Validation**: Validator crate
- **Component System**: Rust-form components

### Frontend (Generated by Rust-form)
- **Framework**: React 18 with TypeScript
- **State Management**: React Query + Zustand
- **Styling**: Tailwind CSS
- **Drag & Drop**: @dnd-kit/core
- **Forms**: React Hook Form
- **Code Editor**: Monaco Editor
- **Build Tool**: Vite

### Additional Libraries
- **YAML Processing**: js-yaml
- **JSON Schema**: ajv
- **File Handling**: file-saver
- **Icons**: Heroicons
- **Notifications**: react-hot-toast

## Key Features

### 1. Visual Model Designer
- Drag models from palette
- Visual field editor
- Relationship mapping
- Index configuration
- Constraint management

### 2. Endpoint Configuration
- Visual CRUD builder
- Authentication settings
- Filter configuration
- Pagination setup

### 3. Component Marketplace
- Browse available components
- Preview component templates
- Dependency management
- Version selection

### 4. Live Preview
- Real-time YAML generation
- Validation feedback
- Error highlighting
- Diff visualization

### 5. Project Generation
- One-click generation
- Progress tracking
- File management
- Download packaging

## Development Phases

### Phase 1: Core Infrastructure (MVP)
- Basic CRUD for configurations
- Simple form-based editor
- YAML export/import
- Basic validation

### Phase 2: Visual Interface
- Drag-and-drop system
- Component palette
- Visual model designer
- Real-time preview

### Phase 3: Component Integration
- Component discovery
- Installation system
- Template library
- Marketplace features

### Phase 4: Advanced Features
- Project generation
- File management
- Collaboration features
- Template sharing

## Benefits of This Architecture

1. **Dogfooding**: Showcases Rust-form capabilities
2. **User-Friendly**: Visual interface lowers barrier to entry  
3. **Educational**: Demonstrates best practices
4. **Extensible**: Component system allows growth
5. **Self-Contained**: Single deployable application
6. **Performance**: Rust backend ensures speed

This architecture provides a solid foundation for building a comprehensive visual configuration editor that demonstrates the full power of the Rust-form framework while providing real value to users.