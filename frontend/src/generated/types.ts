// Generated TypeScript types for todo_api
// This file is auto-generated. Do not edit manually.


// Todo type definitions
export interface Todo {
  id: number;
  title: string;
  description: string;
  completed: boolean;
  priority: string;
  due_date: string | null;
  created_at: string;
  updated_at: string;
  
}

export interface CreateTodoRequest {
  title: string;
  description?: string;
  completed?: boolean;
  priority?: string;
  due_date?: string | null;
  
}

export interface UpdateTodoRequest {
  title?: string;
  description?: string;
  completed?: boolean;
  priority?: string;
  due_date?: string | null;
  
}



// API Response types
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface ApiError {
  error: string;
  message: string;
  status: number;
}

// Common utility types
export type ID = string;
export type Timestamp = string;

// Filter types for API queries

export interface TodoFilters {
  id?: number;
  title?: string;
  description?: string;
  completed?: boolean;
  priority?: string;
  due_date?: string | null;
  created_at?: string;
  updated_at?: string;
  
  // Pagination
  page?: number;
  per_page?: number;
  // Sorting
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

