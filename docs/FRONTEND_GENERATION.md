# Frontend Generation Guide

Comprehensive guide to generating modern, type-safe frontend applications with Rust-form.

## 🎯 Overview

Rust-form generates complete frontend applications from your YAML configuration, with perfect type safety between your Rust backend and TypeScript frontend.

## 🚀 Quick Start

```yaml
frontend:
  target: "react"
  typescript_output_dir: "../frontend/src/generated"
  generate_ui_for: ["User", "Post"]
```

```bash
rustform generate config.yml
cd my_app/frontend && npm install && npm run dev
```

## 🎨 Supported Frameworks

### React (Available Now)
- **TypeScript** with perfect backend integration
- **React Query** for state management and caching
- **React Hook Form** with Zod validation
- **Tailwind CSS** for styling

### Vue (Coming Soon)
- **Vue 3** with Composition API
- **Pinia** for state management
- **VeeValidate** for forms
- **Tailwind CSS** for styling

### Svelte (Coming Soon)
- **SvelteKit** for full-stack development
- **Svelte Store** for state management
- **Tailwind CSS** for styling

## 🔧 Configuration

### Basic Setup

```yaml
frontend:
  target: "react"                           # Framework choice
  typescript_output_dir: "../frontend/src/generated"
  generate_ui_for: ["User", "Post"]         # Opt-in generation
  
  components:
    User:
      form_fields: ["name", "email"]        # Customize forms
      list_columns: ["name", "email", "created_at"]
      features: ["create", "edit", "delete", "search"]
      
  framework_config:
    react:
      state_management: "react-query"
      styling: "tailwind"
      forms: "react-hook-form"
```

### Advanced Configuration

```yaml
frontend:
  target: "react"
  typescript_output_dir: "../frontend/src/generated"
  generate_ui_for: ["User", "Post", "Category"]
  
  # Global settings
  auto_generate_types: true
  api_base_url: "http://localhost:8080"
  
  # Component customization
  components:
    User:
      generate: ["form", "list", "card"]     # Choose components
      form_fields: ["name", "email", "avatar"]
      list_columns: ["name", "email", "created_at", "status"]
      features: ["create", "edit", "delete", "search", "export"]
      pagination: true
      search_fields: ["name", "email"]
      
    Post:
      generate: ["form", "list"]
      form_fields: ["title", "content", "category_id", "published"]
      list_columns: ["title", "author", "category", "published", "created_at"]
      features: ["create", "edit", "delete", "search", "filter"]
      relationships:
        author: { display_field: "name" }
        category: { display_field: "name" }
  
  # Framework-specific configuration
  framework_config:
    react:
      version: "18"
      typescript: true
      state_management: "react-query"
      styling: "tailwind"
      forms: "react-hook-form"
      routing: "react-router"
      build_tool: "vite"
```

## 🏗️ Generated Structure

```
frontend/
├── src/
│   ├── generated/
│   │   ├── types/                 # Auto-generated TypeScript types
│   │   │   ├── models.ts         # Backend model interfaces
│   │   │   └── api.ts            # API request/response types
│   │   ├── api/
│   │   │   ├── client.ts         # Type-safe API client
│   │   │   └── hooks.ts          # React Query hooks
│   │   └── components/           # Generated UI components
│   │       ├── User/
│   │       │   ├── UserForm.tsx
│   │       │   ├── UserList.tsx
│   │       │   ├── UserCard.tsx
│   │       │   └── index.ts
│   │       └── Post/
│   ├── components/               # Custom components
│   ├── pages/                    # Application pages
│   ├── hooks/                    # Custom hooks
│   └── App.tsx
├── package.json
├── tailwind.config.js
└── vite.config.ts
```

## 🔄 Type Safety Pipeline

### 1. Backend Type Definition
```rust
// Generated in Rust backend
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../frontend/src/generated/types/")]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

### 2. Auto-Generated TypeScript
```typescript
// Auto-generated types/models.ts
export interface User {
  id: string;
  name: string;
  email: string;
  created_at: string;
}

export interface CreateUserRequest {
  name: string;
  email: string;
}
```

### 3. Type-Safe Components
```tsx
// Generated UserForm.tsx
import { User, CreateUserRequest } from '../types/models';

interface UserFormProps {
  user?: User;
  onSubmit: (data: CreateUserRequest) => void;
}

export function UserForm({ user, onSubmit }: UserFormProps) {
  const { register, handleSubmit } = useForm<CreateUserRequest>();
  // TypeScript ensures field names match backend exactly!
}
```

## 🎨 React Generation

### Generated Components

#### Form Components
```tsx
// UserForm.tsx - Auto-generated with validation
export function UserForm({ user, onSubmit, onCancel }: UserFormProps) {
  const { register, handleSubmit, formState: { errors } } = useForm<CreateUserRequest>({
    resolver: zodResolver(createUserSchema),
    defaultValues: user
  });

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div>
        <label className="block text-sm font-medium">Name</label>
        <input
          {...register("name")}
          className="mt-1 block w-full rounded-md border-gray-300"
        />
        {errors.name && <p className="text-red-500 text-sm">{errors.name.message}</p>}
      </div>
      
      <div className="flex gap-2">
        <button type="submit" className="btn btn-primary">Save</button>
        <button type="button" onClick={onCancel} className="btn btn-secondary">Cancel</button>
      </div>
    </form>
  );
}
```

#### List Components
```tsx
// UserList.tsx - Auto-generated with pagination
export function UserList() {
  const { data, isLoading, error } = useUsers();
  const deleteUser = useDeleteUser();

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Users</h2>
        <UserCreateButton />
      </div>
      
      <div className="overflow-x-auto">
        <table className="min-w-full table-auto">
          <thead>
            <tr>
              <th>Name</th>
              <th>Email</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {data?.map(user => (
              <tr key={user.id}>
                <td>{user.name}</td>
                <td>{user.email}</td>
                <td>{formatDate(user.created_at)}</td>
                <td>
                  <UserEditButton user={user} />
                  <UserDeleteButton user={user} onDelete={deleteUser.mutate} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
```

### API Integration
```tsx
// Generated hooks using React Query
export function useUsers() {
  return useQuery({
    queryKey: ['users'],
    queryFn: () => apiClient.get<User[]>('/users')
  });
}

export function useCreateUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateUserRequest) => 
      apiClient.post<User>('/users', data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['users'] });
    }
  });
}
```

## 🎯 Framework Switching

### From React to Vue

**Before:**
```yaml
frontend:
  target: "react"
  generate_ui_for: ["User"]
```

**After:**
```yaml
frontend:
  target: "vue"
  generate_ui_for: ["User"]
```

```bash
rustform generate config.yml
cd frontend && npm install && npm run dev
```

**Result:** Same functionality, Vue implementation!

## 🛠️ Customization

### Custom Components

1. **Start with generated components**
2. **Copy to custom directory**
3. **Remove from generate_ui_for**
4. **Customize as needed**

```bash
# Copy generated component
cp src/generated/components/User/UserForm.tsx src/components/
```

```yaml
# Remove from generation
frontend:
  generate_ui_for: ["Post"]  # Removed "User"
```

### Component Library Export

```yaml
frontend:
  export:
    package_name: "@myapp/components"
    version: "1.0.0"
    components: ["UserForm", "PostList"]
```

Generates npm package structure for sharing components between projects.

## 🔧 Development Workflow

### 1. Development Mode
```bash
# Watch for backend changes and regenerate types
cd backend && cargo watch -x check

# Frontend development server
cd frontend && npm run dev
```

### 2. Type Regeneration
```bash
# Force type regeneration
cd backend && cargo check

# Types automatically updated in frontend/src/generated/types/
```

### 3. Adding New Models
```yaml
# Add new model to config
models:
  Comment:
    fields:
      id: { type: "uuid", primary_key: true }
      content: { type: "text" }
      post_id: { type: "uuid", references: "Post.id" }

# Add to frontend generation
frontend:
  generate_ui_for: ["User", "Post", "Comment"]
```

```bash
rustform generate config.yml
# Comment components automatically available!
```

## 🎨 Styling and Theming

### Tailwind CSS (Default)
```tsx
// Generated with Tailwind classes
<button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
  Save
</button>
```

### Material-UI (Future)
```yaml
frontend:
  framework_config:
    react:
      styling: "mui"
```

### Styled Components (Future)
```yaml
frontend:
  framework_config:
    react:
      styling: "styled-components"
```

## 📱 Responsive Design

Generated components are mobile-first and responsive:

```tsx
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  {/* Responsive grid layout */}
</div>

<div className="overflow-x-auto">
  <table className="min-w-full">
    {/* Scrollable table on mobile */}
  </table>
</div>
```

## 🧪 Testing

### Generated Test Structure
```
frontend/
├── src/
│   ├── generated/
│   │   └── components/
│   │       └── User/
│   │           ├── UserForm.tsx
│   │           ├── UserForm.test.tsx
│   │           └── UserList.test.tsx
```

### Example Test
```tsx
// UserForm.test.tsx
import { render, screen } from '@testing-library/react';
import { UserForm } from './UserForm';

test('renders user form fields', () => {
  render(<UserForm onSubmit={jest.fn()} />);
  
  expect(screen.getByLabelText(/name/i)).toBeInTheDocument();
  expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
});
```

## 🚀 Production Deployment

### Build Process
```bash
cd frontend
npm run build

# Optimized production build in dist/
```

### Environment Variables
```typescript
// Generated with environment configuration
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
```

## 🔮 Future Features

- **Vue 3 Support** - Complete Vue.js integration
- **Svelte Support** - SvelteKit applications  
- **Angular Support** - Angular with TypeScript
- **Mobile Generation** - React Native components
- **Desktop Generation** - Tauri applications
- **Advanced Validation** - Complex validation rules
- **Real-time Features** - WebSocket integration
- **Offline Support** - PWA capabilities

Frontend generation makes Rust-form a complete full-stack solution! 🚀