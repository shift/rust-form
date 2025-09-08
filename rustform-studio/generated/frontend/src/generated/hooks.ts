// Generated React Query hooks for rustform_studio
// This file is auto-generated. Do not edit manually.

import { 
  useQuery, 
  useMutation, 
  useQueryClient,
  UseQueryOptions,
  UseMutationOptions,
  QueryKey,
} from '@tanstack/react-query';
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
import { apiClient } from './api-client';

// Query key factories
export const queryKeys = {
  config: {
    all: ['configs'] as const,
    lists: () => [...queryKeys.config.all, 'list'] as const,
    list: (filters: Partial<ConfigFilters>) => [...queryKeys.config.lists(), filters] as const,
    details: () => [...queryKeys.config.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.config.details(), id] as const,
  },
  component: {
    all: ['components'] as const,
    lists: () => [...queryKeys.component.all, 'list'] as const,
    list: (filters: Partial<ComponentFilters>) => [...queryKeys.component.lists(), filters] as const,
    details: () => [...queryKeys.component.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.component.details(), id] as const,
  },
  project: {
    all: ['projects'] as const,
    lists: () => [...queryKeys.project.all, 'list'] as const,
    list: (filters: Partial<ProjectFilters>) => [...queryKeys.project.lists(), filters] as const,
    details: () => [...queryKeys.project.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.project.details(), id] as const,
  },
  template: {
    all: ['templates'] as const,
    lists: () => [...queryKeys.template.all, 'list'] as const,
    list: (filters: Partial<TemplateFilters>) => [...queryKeys.template.lists(), filters] as const,
    details: () => [...queryKeys.template.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.template.details(), id] as const,
  },
  };


// Config hooks
export function useConfigs(
  params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' },
  options?: Omit<UseQueryOptions<PaginatedResponse<Config>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.config.list(params || {}),
    queryFn: () => apiClient.config.getAll(params),
    ...options,
  });
}

export function useConfig(
  id: string,
  options?: Omit<UseQueryOptions<Config, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.config.detail(id),
    queryFn: () => apiClient.config.getById(id),
    enabled: !!id,
    ...options,
  });
}

export function useSearchConfigs(
  filters: Partial<ConfigFilters>,
  options?: Omit<UseQueryOptions<PaginatedResponse<Config>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.config.list(filters),
    queryFn: () => apiClient.config.search(filters),
    ...options,
  });
}

export function useCreateConfig(
  options?: UseMutationOptions<Config, ApiError, CreateConfigRequest>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateConfigRequest) => apiClient.config.create(data),
    onSuccess: (data) => {
      // Invalidate and refetch config lists
      queryClient.invalidateQueries({ queryKey: queryKeys.config.lists() });
      // Add the new config to the cache
      queryClient.setQueryData(queryKeys.config.detail(data.id), data);
    },
    ...options,
  });
}

export function useUpdateConfig(
  options?: UseMutationOptions<Config, ApiError, { id: string; data: UpdateConfigRequest }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.config.update(id, data),
    onSuccess: (data, variables) => {
      // Update the config in the cache
      queryClient.setQueryData(queryKeys.config.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.config.lists() });
    },
    ...options,
  });
}

export function usePatchConfig(
  options?: UseMutationOptions<Config, ApiError, { id: string; data: Partial<UpdateConfigRequest> }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.config.patch(id, data),
    onSuccess: (data, variables) => {
      // Update the config in the cache
      queryClient.setQueryData(queryKeys.config.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.config.lists() });
    },
    ...options,
  });
}

export function useDeleteConfig(
  options?: UseMutationOptions<void, ApiError, string>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (id: string) => apiClient.config.delete(id),
    onSuccess: (_, id) => {
      // Remove the config from the cache
      queryClient.removeQueries({ queryKey: queryKeys.config.detail(id) });
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.config.lists() });
    },
    ...options,
  });
}


// Component hooks
export function useComponents(
  params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' },
  options?: Omit<UseQueryOptions<PaginatedResponse<Component>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.component.list(params || {}),
    queryFn: () => apiClient.component.getAll(params),
    ...options,
  });
}

export function useComponent(
  id: string,
  options?: Omit<UseQueryOptions<Component, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.component.detail(id),
    queryFn: () => apiClient.component.getById(id),
    enabled: !!id,
    ...options,
  });
}

export function useSearchComponents(
  filters: Partial<ComponentFilters>,
  options?: Omit<UseQueryOptions<PaginatedResponse<Component>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.component.list(filters),
    queryFn: () => apiClient.component.search(filters),
    ...options,
  });
}

export function useCreateComponent(
  options?: UseMutationOptions<Component, ApiError, CreateComponentRequest>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateComponentRequest) => apiClient.component.create(data),
    onSuccess: (data) => {
      // Invalidate and refetch component lists
      queryClient.invalidateQueries({ queryKey: queryKeys.component.lists() });
      // Add the new component to the cache
      queryClient.setQueryData(queryKeys.component.detail(data.id), data);
    },
    ...options,
  });
}

export function useUpdateComponent(
  options?: UseMutationOptions<Component, ApiError, { id: string; data: UpdateComponentRequest }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.component.update(id, data),
    onSuccess: (data, variables) => {
      // Update the component in the cache
      queryClient.setQueryData(queryKeys.component.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.component.lists() });
    },
    ...options,
  });
}

export function usePatchComponent(
  options?: UseMutationOptions<Component, ApiError, { id: string; data: Partial<UpdateComponentRequest> }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.component.patch(id, data),
    onSuccess: (data, variables) => {
      // Update the component in the cache
      queryClient.setQueryData(queryKeys.component.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.component.lists() });
    },
    ...options,
  });
}

export function useDeleteComponent(
  options?: UseMutationOptions<void, ApiError, string>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (id: string) => apiClient.component.delete(id),
    onSuccess: (_, id) => {
      // Remove the component from the cache
      queryClient.removeQueries({ queryKey: queryKeys.component.detail(id) });
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.component.lists() });
    },
    ...options,
  });
}


// Project hooks
export function useProjects(
  params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' },
  options?: Omit<UseQueryOptions<PaginatedResponse<Project>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.project.list(params || {}),
    queryFn: () => apiClient.project.getAll(params),
    ...options,
  });
}

export function useProject(
  id: string,
  options?: Omit<UseQueryOptions<Project, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.project.detail(id),
    queryFn: () => apiClient.project.getById(id),
    enabled: !!id,
    ...options,
  });
}

export function useSearchProjects(
  filters: Partial<ProjectFilters>,
  options?: Omit<UseQueryOptions<PaginatedResponse<Project>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.project.list(filters),
    queryFn: () => apiClient.project.search(filters),
    ...options,
  });
}

export function useCreateProject(
  options?: UseMutationOptions<Project, ApiError, CreateProjectRequest>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateProjectRequest) => apiClient.project.create(data),
    onSuccess: (data) => {
      // Invalidate and refetch project lists
      queryClient.invalidateQueries({ queryKey: queryKeys.project.lists() });
      // Add the new project to the cache
      queryClient.setQueryData(queryKeys.project.detail(data.id), data);
    },
    ...options,
  });
}

export function useUpdateProject(
  options?: UseMutationOptions<Project, ApiError, { id: string; data: UpdateProjectRequest }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.project.update(id, data),
    onSuccess: (data, variables) => {
      // Update the project in the cache
      queryClient.setQueryData(queryKeys.project.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.project.lists() });
    },
    ...options,
  });
}

export function usePatchProject(
  options?: UseMutationOptions<Project, ApiError, { id: string; data: Partial<UpdateProjectRequest> }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.project.patch(id, data),
    onSuccess: (data, variables) => {
      // Update the project in the cache
      queryClient.setQueryData(queryKeys.project.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.project.lists() });
    },
    ...options,
  });
}

export function useDeleteProject(
  options?: UseMutationOptions<void, ApiError, string>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (id: string) => apiClient.project.delete(id),
    onSuccess: (_, id) => {
      // Remove the project from the cache
      queryClient.removeQueries({ queryKey: queryKeys.project.detail(id) });
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.project.lists() });
    },
    ...options,
  });
}


// Template hooks
export function useTemplates(
  params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' },
  options?: Omit<UseQueryOptions<PaginatedResponse<Template>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.template.list(params || {}),
    queryFn: () => apiClient.template.getAll(params),
    ...options,
  });
}

export function useTemplate(
  id: string,
  options?: Omit<UseQueryOptions<Template, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.template.detail(id),
    queryFn: () => apiClient.template.getById(id),
    enabled: !!id,
    ...options,
  });
}

export function useSearchTemplates(
  filters: Partial<TemplateFilters>,
  options?: Omit<UseQueryOptions<PaginatedResponse<Template>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.template.list(filters),
    queryFn: () => apiClient.template.search(filters),
    ...options,
  });
}

export function useCreateTemplate(
  options?: UseMutationOptions<Template, ApiError, CreateTemplateRequest>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateTemplateRequest) => apiClient.template.create(data),
    onSuccess: (data) => {
      // Invalidate and refetch template lists
      queryClient.invalidateQueries({ queryKey: queryKeys.template.lists() });
      // Add the new template to the cache
      queryClient.setQueryData(queryKeys.template.detail(data.id), data);
    },
    ...options,
  });
}

export function useUpdateTemplate(
  options?: UseMutationOptions<Template, ApiError, { id: string; data: UpdateTemplateRequest }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.template.update(id, data),
    onSuccess: (data, variables) => {
      // Update the template in the cache
      queryClient.setQueryData(queryKeys.template.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.template.lists() });
    },
    ...options,
  });
}

export function usePatchTemplate(
  options?: UseMutationOptions<Template, ApiError, { id: string; data: Partial<UpdateTemplateRequest> }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.template.patch(id, data),
    onSuccess: (data, variables) => {
      // Update the template in the cache
      queryClient.setQueryData(queryKeys.template.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.template.lists() });
    },
    ...options,
  });
}

export function useDeleteTemplate(
  options?: UseMutationOptions<void, ApiError, string>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (id: string) => apiClient.template.delete(id),
    onSuccess: (_, id) => {
      // Remove the template from the cache
      queryClient.removeQueries({ queryKey: queryKeys.template.detail(id) });
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.template.lists() });
    },
    ...options,
  });
}



// Bulk operations and utilities
export function useInvalidateQueries() {
  const queryClient = useQueryClient();
  
  return {
    config: {
      all: () => queryClient.invalidateQueries({ queryKey: queryKeys.config.all }),
      lists: () => queryClient.invalidateQueries({ queryKey: queryKeys.config.lists() }),
      detail: (id: string) => queryClient.invalidateQueries({ queryKey: queryKeys.config.detail(id) }),
    },
    component: {
      all: () => queryClient.invalidateQueries({ queryKey: queryKeys.component.all }),
      lists: () => queryClient.invalidateQueries({ queryKey: queryKeys.component.lists() }),
      detail: (id: string) => queryClient.invalidateQueries({ queryKey: queryKeys.component.detail(id) }),
    },
    project: {
      all: () => queryClient.invalidateQueries({ queryKey: queryKeys.project.all }),
      lists: () => queryClient.invalidateQueries({ queryKey: queryKeys.project.lists() }),
      detail: (id: string) => queryClient.invalidateQueries({ queryKey: queryKeys.project.detail(id) }),
    },
    template: {
      all: () => queryClient.invalidateQueries({ queryKey: queryKeys.template.all }),
      lists: () => queryClient.invalidateQueries({ queryKey: queryKeys.template.lists() }),
      detail: (id: string) => queryClient.invalidateQueries({ queryKey: queryKeys.template.detail(id) }),
    },
    };
}

// Optimistic updates helper
export function useOptimisticUpdate() {
  const queryClient = useQueryClient();
  
  return {
    config: {
      update: (id: string, updater: (old: Config) => Config) => {
        queryClient.setQueryData(
          queryKeys.config.detail(id),
          updater
        );
      },
      create: (newConfig: Config) => {
        queryClient.setQueryData(
          queryKeys.config.detail(newConfig.id),
          newConfig
        );
      },
    },
    component: {
      update: (id: string, updater: (old: Component) => Component) => {
        queryClient.setQueryData(
          queryKeys.component.detail(id),
          updater
        );
      },
      create: (newComponent: Component) => {
        queryClient.setQueryData(
          queryKeys.component.detail(newComponent.id),
          newComponent
        );
      },
    },
    project: {
      update: (id: string, updater: (old: Project) => Project) => {
        queryClient.setQueryData(
          queryKeys.project.detail(id),
          updater
        );
      },
      create: (newProject: Project) => {
        queryClient.setQueryData(
          queryKeys.project.detail(newProject.id),
          newProject
        );
      },
    },
    template: {
      update: (id: string, updater: (old: Template) => Template) => {
        queryClient.setQueryData(
          queryKeys.template.detail(id),
          updater
        );
      },
      create: (newTemplate: Template) => {
        queryClient.setQueryData(
          queryKeys.template.detail(newTemplate.id),
          newTemplate
        );
      },
    },
    };
}

// Prefetch utilities
export function usePrefetch() {
  const queryClient = useQueryClient();
  
  return {
    config: {
      list: (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.config.list(params || {}),
          queryFn: () => apiClient.config.getAll(params),
        });
      },
      detail: (id: string) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.config.detail(id),
          queryFn: () => apiClient.config.getById(id),
        });
      },
    },
    component: {
      list: (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.component.list(params || {}),
          queryFn: () => apiClient.component.getAll(params),
        });
      },
      detail: (id: string) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.component.detail(id),
          queryFn: () => apiClient.component.getById(id),
        });
      },
    },
    project: {
      list: (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.project.list(params || {}),
          queryFn: () => apiClient.project.getAll(params),
        });
      },
      detail: (id: string) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.project.detail(id),
          queryFn: () => apiClient.project.getById(id),
        });
      },
    },
    template: {
      list: (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.template.list(params || {}),
          queryFn: () => apiClient.template.getAll(params),
        });
      },
      detail: (id: string) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.template.detail(id),
          queryFn: () => apiClient.template.getById(id),
        });
      },
    },
    };
}