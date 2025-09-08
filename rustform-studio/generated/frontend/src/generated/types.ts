// Generated TypeScript types for rustform_studio
// This file is auto-generated. Do not edit manually.


// Config type definitions
export interface Config {
  id: number;
  name: string;
  description: string;
  yaml_content: string;
  is_template: boolean;
  created_at: string;
  updated_at: string;
  
}

export interface CreateConfigRequest {
  name: string;
  description?: string;
  yaml_content: string;
  is_template?: boolean;
  
}

export interface UpdateConfigRequest {
  name?: string;
  description?: string;
  yaml_content?: string;
  is_template?: boolean;
  
}


// Component type definitions
export interface Component {
  id: number;
  name: string;
  uri: string;
  manifest_data: any;
  description: string;
  version: string;
  author: string;
  keywords: any;
  cached_at: string;
  
}

export interface CreateComponentRequest {
  name: string;
  uri: string;
  manifest_data?: any;
  description?: string;
  version?: string;
  author?: string;
  keywords?: any;
  
}

export interface UpdateComponentRequest {
  name?: string;
  uri?: string;
  manifest_data?: any;
  description?: string;
  version?: string;
  author?: string;
  keywords?: any;
  
}


// Project type definitions
export interface Project {
  id: number;
  name: string;
  config_id: number;
  generated_at: string;
  file_path: string;
  generation_log: string;
  status: string;
  
}

export interface CreateProjectRequest {
  name: string;
  config_id: number;
  file_path?: string;
  generation_log?: string;
  status?: string;
  
}

export interface UpdateProjectRequest {
  name?: string;
  config_id?: number;
  file_path?: string;
  generation_log?: string;
  status?: string;
  
}


// Template type definitions
export interface Template {
  id: number;
  name: string;
  category: string;
  description: string;
  yaml_content: string;
  tags: any;
  is_public: boolean;
  created_at: string;
  
}

export interface CreateTemplateRequest {
  name: string;
  category: string;
  description?: string;
  yaml_content: string;
  tags?: any;
  is_public?: boolean;
  
}

export interface UpdateTemplateRequest {
  name?: string;
  category?: string;
  description?: string;
  yaml_content?: string;
  tags?: any;
  is_public?: boolean;
  
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

export interface ConfigFilters {
  id?: number;
  name?: string;
  description?: string;
  yaml_content?: string;
  is_template?: boolean;
  created_at?: string;
  updated_at?: string;
  
  // Pagination
  page?: number;
  per_page?: number;
  // Sorting
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}


export interface ComponentFilters {
  id?: number;
  name?: string;
  uri?: string;
  manifest_data?: any;
  description?: string;
  version?: string;
  author?: string;
  keywords?: any;
  cached_at?: string;
  
  // Pagination
  page?: number;
  per_page?: number;
  // Sorting
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}


export interface ProjectFilters {
  id?: number;
  name?: string;
  config_id?: number;
  generated_at?: string;
  file_path?: string;
  generation_log?: string;
  status?: string;
  
  // Pagination
  page?: number;
  per_page?: number;
  // Sorting
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}


export interface TemplateFilters {
  id?: number;
  name?: string;
  category?: string;
  description?: string;
  yaml_content?: string;
  tags?: any;
  is_public?: boolean;
  created_at?: string;
  
  // Pagination
  page?: number;
  per_page?: number;
  // Sorting
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

