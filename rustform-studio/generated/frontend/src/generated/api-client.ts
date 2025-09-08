// Generated API client for rustform_studio
// This file is auto-generated. Do not edit manually.

import { 
  Config, 
  CreateConfigRequest, 
  UpdateConfigRequest,
  ConfigFilters,
  Component, 
  CreateComponentRequest, 
  UpdateComponentRequest,
  ComponentFilters,
  Project, 
  CreateProjectRequest, 
  UpdateProjectRequest,
  ProjectFilters,
  Template, 
  CreateTemplateRequest, 
  UpdateTemplateRequest,
  TemplateFilters,
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

  
  // Config API methods
  config = {
    // Get all configs
    getAll: async (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }): Promise<PaginatedResponse<Config>> => {
      const query = new URLSearchParams();
      if (params?.page) query.append('page', params.page.toString());
      if (params?.per_page) query.append('per_page', params.per_page.toString());
      if (params?.sort_by) query.append('sort_by', params.sort_by);
      if (params?.sort_order) query.append('sort_order', params.sort_order);
      
      return this.request<PaginatedResponse<Config>>(
        `/api/configs${query.toString() ? '?' + query.toString() : ''}`
      );
    },

    // Get config by ID
    getById: async (id: string): Promise<Config> => {
      return this.request<Config>(`/api/configs/${id}`);
    },

    // Create new config
    create: async (data: CreateConfigRequest): Promise<Config> => {
      return this.request<Config>(`/api/configs`, {
        method: 'POST',
        body: JSON.stringify(data),
      });
    },

    // Update config
    update: async (id: string, data: UpdateConfigRequest): Promise<Config> => {
      return this.request<Config>(`/api/configs/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      });
    },

    // Partially update config
    patch: async (id: string, data: Partial<UpdateConfigRequest>): Promise<Config> => {
      return this.request<Config>(`/api/configs/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
      });
    },

    // Delete config
    delete: async (id: string): Promise<void> => {
      await this.request<void>(`/api/configs/${id}`, {
        method: 'DELETE',
      });
    },

    // Search configs
    search: async (filters: Partial<ConfigFilters>): Promise<PaginatedResponse<Config>> => {
      const query = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          query.append(key, value.toString());
        }
      });
      
      return this.request<PaginatedResponse<Config>>(
        `/api/configs/search${query.toString() ? '?' + query.toString() : ''}`
      );
    },
  };

  
  // Component API methods
  component = {
    // Get all components
    getAll: async (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }): Promise<PaginatedResponse<Component>> => {
      const query = new URLSearchParams();
      if (params?.page) query.append('page', params.page.toString());
      if (params?.per_page) query.append('per_page', params.per_page.toString());
      if (params?.sort_by) query.append('sort_by', params.sort_by);
      if (params?.sort_order) query.append('sort_order', params.sort_order);
      
      return this.request<PaginatedResponse<Component>>(
        `/api/components${query.toString() ? '?' + query.toString() : ''}`
      );
    },

    // Get component by ID
    getById: async (id: string): Promise<Component> => {
      return this.request<Component>(`/api/components/${id}`);
    },

    // Create new component
    create: async (data: CreateComponentRequest): Promise<Component> => {
      return this.request<Component>(`/api/components`, {
        method: 'POST',
        body: JSON.stringify(data),
      });
    },

    // Update component
    update: async (id: string, data: UpdateComponentRequest): Promise<Component> => {
      return this.request<Component>(`/api/components/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      });
    },

    // Partially update component
    patch: async (id: string, data: Partial<UpdateComponentRequest>): Promise<Component> => {
      return this.request<Component>(`/api/components/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
      });
    },

    // Delete component
    delete: async (id: string): Promise<void> => {
      await this.request<void>(`/api/components/${id}`, {
        method: 'DELETE',
      });
    },

    // Search components
    search: async (filters: Partial<ComponentFilters>): Promise<PaginatedResponse<Component>> => {
      const query = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          query.append(key, value.toString());
        }
      });
      
      return this.request<PaginatedResponse<Component>>(
        `/api/components/search${query.toString() ? '?' + query.toString() : ''}`
      );
    },
  };

  
  // Project API methods
  project = {
    // Get all projects
    getAll: async (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }): Promise<PaginatedResponse<Project>> => {
      const query = new URLSearchParams();
      if (params?.page) query.append('page', params.page.toString());
      if (params?.per_page) query.append('per_page', params.per_page.toString());
      if (params?.sort_by) query.append('sort_by', params.sort_by);
      if (params?.sort_order) query.append('sort_order', params.sort_order);
      
      return this.request<PaginatedResponse<Project>>(
        `/api/projects${query.toString() ? '?' + query.toString() : ''}`
      );
    },

    // Get project by ID
    getById: async (id: string): Promise<Project> => {
      return this.request<Project>(`/api/projects/${id}`);
    },

    // Create new project
    create: async (data: CreateProjectRequest): Promise<Project> => {
      return this.request<Project>(`/api/projects`, {
        method: 'POST',
        body: JSON.stringify(data),
      });
    },

    // Update project
    update: async (id: string, data: UpdateProjectRequest): Promise<Project> => {
      return this.request<Project>(`/api/projects/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      });
    },

    // Partially update project
    patch: async (id: string, data: Partial<UpdateProjectRequest>): Promise<Project> => {
      return this.request<Project>(`/api/projects/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
      });
    },

    // Delete project
    delete: async (id: string): Promise<void> => {
      await this.request<void>(`/api/projects/${id}`, {
        method: 'DELETE',
      });
    },

    // Search projects
    search: async (filters: Partial<ProjectFilters>): Promise<PaginatedResponse<Project>> => {
      const query = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          query.append(key, value.toString());
        }
      });
      
      return this.request<PaginatedResponse<Project>>(
        `/api/projects/search${query.toString() ? '?' + query.toString() : ''}`
      );
    },
  };

  
  // Template API methods
  template = {
    // Get all templates
    getAll: async (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }): Promise<PaginatedResponse<Template>> => {
      const query = new URLSearchParams();
      if (params?.page) query.append('page', params.page.toString());
      if (params?.per_page) query.append('per_page', params.per_page.toString());
      if (params?.sort_by) query.append('sort_by', params.sort_by);
      if (params?.sort_order) query.append('sort_order', params.sort_order);
      
      return this.request<PaginatedResponse<Template>>(
        `/api/templates${query.toString() ? '?' + query.toString() : ''}`
      );
    },

    // Get template by ID
    getById: async (id: string): Promise<Template> => {
      return this.request<Template>(`/api/templates/${id}`);
    },

    // Create new template
    create: async (data: CreateTemplateRequest): Promise<Template> => {
      return this.request<Template>(`/api/templates`, {
        method: 'POST',
        body: JSON.stringify(data),
      });
    },

    // Update template
    update: async (id: string, data: UpdateTemplateRequest): Promise<Template> => {
      return this.request<Template>(`/api/templates/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      });
    },

    // Partially update template
    patch: async (id: string, data: Partial<UpdateTemplateRequest>): Promise<Template> => {
      return this.request<Template>(`/api/templates/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
      });
    },

    // Delete template
    delete: async (id: string): Promise<void> => {
      await this.request<void>(`/api/templates/${id}`, {
        method: 'DELETE',
      });
    },

    // Search templates
    search: async (filters: Partial<TemplateFilters>): Promise<PaginatedResponse<Template>> => {
      const query = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          query.append(key, value.toString());
        }
      });
      
      return this.request<PaginatedResponse<Template>>(
        `/api/templates/search${query.toString() ? '?' + query.toString() : ''}`
      );
    },
  };

  
}

// Default API client instance
export const apiClient = new ApiClient({
  baseUrl: process.env.NODE_ENV === 'development' 
    ? 'http://localhost:4000'
    : '/api',
});
// Type-safe wrapper functions for easier usage

export const configApi = apiClient.config;

export const componentApi = apiClient.component;

export const projectApi = apiClient.project;

export const templateApi = apiClient.template;
