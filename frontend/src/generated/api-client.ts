// Generated API client for todo_api
// This file is auto-generated. Do not edit manually.

import { 
  Todo, 
  CreateTodoRequest, 
  UpdateTodoRequest,
  TodoFilters,
  PaginatedResponse,
  ApiError,
} from './types';

export interface ApiClientConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public response?: any
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export class ApiClient {
  private baseUrl: string;
  private headers: Record<string, string>;

  constructor(config: ApiClientConfig) {
    this.baseUrl = config.baseUrl.replace(/\/$/, ''); // Remove trailing slash
    this.headers = {
      'Content-Type': 'application/json',
      ...config.headers,
    };
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    const config: RequestInit = {
      headers: this.headers,
      ...options,
    };

    try {
      const response = await fetch(url, config);
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new ApiError(
          errorData.message || `HTTP ${response.status}: ${response.statusText}`,
          response.status,
          errorData
        );
      }

      return await response.json();
    } catch (error) {
      if (error instanceof ApiError) {
        throw error;
      }
      throw new ApiError(`Network error: ${error.message}`, 0, error);
    }
  }

  // Authentication methods
  setAuthToken(token: string) {
    this.headers['Authorization'] = `Bearer ${token}`;
  }

  removeAuthToken() {
    delete this.headers['Authorization'];
  }

  
  // Todo API methods
  todo = {
    // Get all todos
    getAll: async (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }): Promise<PaginatedResponse<Todo>> => {
      const query = new URLSearchParams();
      if (params?.page) query.append('page', params.page.toString());
      if (params?.per_page) query.append('per_page', params.per_page.toString());
      if (params?.sort_by) query.append('sort_by', params.sort_by);
      if (params?.sort_order) query.append('sort_order', params.sort_order);
      
      return this.request<PaginatedResponse<Todo>>(
        `/api/todos${query.toString() ? '?' + query.toString() : ''}`
      );
    },

    // Get todo by ID
    getById: async (id: string): Promise<Todo> => {
      return this.request<Todo>(`/api/todos/${id}`);
    },

    // Create new todo
    create: async (data: CreateTodoRequest): Promise<Todo> => {
      return this.request<Todo>(`/api/todos`, {
        method: 'POST',
        body: JSON.stringify(data),
      });
    },

    // Update todo
    update: async (id: string, data: UpdateTodoRequest): Promise<Todo> => {
      return this.request<Todo>(`/api/todos/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      });
    },

    // Partially update todo
    patch: async (id: string, data: Partial<UpdateTodoRequest>): Promise<Todo> => {
      return this.request<Todo>(`/api/todos/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
      });
    },

    // Delete todo
    delete: async (id: string): Promise<void> => {
      await this.request<void>(`/api/todos/${id}`, {
        method: 'DELETE',
      });
    },

    // Search todos
    search: async (filters: Partial<TodoFilters>): Promise<PaginatedResponse<Todo>> => {
      const query = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          query.append(key, value.toString());
        }
      });
      
      return this.request<PaginatedResponse<Todo>>(
        `/api/todos/search${query.toString() ? '?' + query.toString() : ''}`
      );
    },
  };

  
}

// Default API client instance
export const apiClient = new ApiClient({
  baseUrl: process.env.NODE_ENV === 'development' 
    ? 'http://localhost:3000'
    : '/api',
});
// Type-safe wrapper functions for easier usage

export const todoApi = apiClient.todo;
