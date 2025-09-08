// Generated React Query hooks for todo_api
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
  Todo, 
  CreateTodoRequest, 
  UpdateTodoRequest,
  TodoFilters,
  PaginatedResponse,
  ApiError,
} from './types';
import { apiClient } from './api-client';

// Query key factories
export const queryKeys = {
  todo: {
    all: ['todos'] as const,
    lists: () => [...queryKeys.todo.all, 'list'] as const,
    list: (filters: Partial<TodoFilters>) => [...queryKeys.todo.lists(), filters] as const,
    details: () => [...queryKeys.todo.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.todo.details(), id] as const,
  },
  };


// Todo hooks
export function useTodos(
  params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' },
  options?: Omit<UseQueryOptions<PaginatedResponse<Todo>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.todo.list(params || {}),
    queryFn: () => apiClient.todo.getAll(params),
    ...options,
  });
}

export function useTodo(
  id: string,
  options?: Omit<UseQueryOptions<Todo, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.todo.detail(id),
    queryFn: () => apiClient.todo.getById(id),
    enabled: !!id,
    ...options,
  });
}

export function useSearchTodos(
  filters: Partial<TodoFilters>,
  options?: Omit<UseQueryOptions<PaginatedResponse<Todo>, ApiError>, 'queryKey' | 'queryFn'>
) {
  return useQuery({
    queryKey: queryKeys.todo.list(filters),
    queryFn: () => apiClient.todo.search(filters),
    ...options,
  });
}

export function useCreateTodo(
  options?: UseMutationOptions<Todo, ApiError, CreateTodoRequest>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateTodoRequest) => apiClient.todo.create(data),
    onSuccess: (data) => {
      // Invalidate and refetch todo lists
      queryClient.invalidateQueries({ queryKey: queryKeys.todo.lists() });
      // Add the new todo to the cache
      queryClient.setQueryData(queryKeys.todo.detail(data.id), data);
    },
    ...options,
  });
}

export function useUpdateTodo(
  options?: UseMutationOptions<Todo, ApiError, { id: string; data: UpdateTodoRequest }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.todo.update(id, data),
    onSuccess: (data, variables) => {
      // Update the todo in the cache
      queryClient.setQueryData(queryKeys.todo.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.todo.lists() });
    },
    ...options,
  });
}

export function usePatchTodo(
  options?: UseMutationOptions<Todo, ApiError, { id: string; data: Partial<UpdateTodoRequest> }>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }) => apiClient.todo.patch(id, data),
    onSuccess: (data, variables) => {
      // Update the todo in the cache
      queryClient.setQueryData(queryKeys.todo.detail(variables.id), data);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.todo.lists() });
    },
    ...options,
  });
}

export function useDeleteTodo(
  options?: UseMutationOptions<void, ApiError, string>
) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (id: string) => apiClient.todo.delete(id),
    onSuccess: (_, id) => {
      // Remove the todo from the cache
      queryClient.removeQueries({ queryKey: queryKeys.todo.detail(id) });
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.todo.lists() });
    },
    ...options,
  });
}



// Bulk operations and utilities
export function useInvalidateQueries() {
  const queryClient = useQueryClient();
  
  return {
    todo: {
      all: () => queryClient.invalidateQueries({ queryKey: queryKeys.todo.all }),
      lists: () => queryClient.invalidateQueries({ queryKey: queryKeys.todo.lists() }),
      detail: (id: string) => queryClient.invalidateQueries({ queryKey: queryKeys.todo.detail(id) }),
    },
    };
}

// Optimistic updates helper
export function useOptimisticUpdate() {
  const queryClient = useQueryClient();
  
  return {
    todo: {
      update: (id: string, updater: (old: Todo) => Todo) => {
        queryClient.setQueryData(
          queryKeys.todo.detail(id),
          updater
        );
      },
      create: (newTodo: Todo) => {
        queryClient.setQueryData(
          queryKeys.todo.detail(newTodo.id),
          newTodo
        );
      },
    },
    };
}

// Prefetch utilities
export function usePrefetch() {
  const queryClient = useQueryClient();
  
  return {
    todo: {
      list: (params?: { page?: number; per_page?: number; sort_by?: string; sort_order?: 'asc' | 'desc' }) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.todo.list(params || {}),
          queryFn: () => apiClient.todo.getAll(params),
        });
      },
      detail: (id: string) => {
        queryClient.prefetchQuery({
          queryKey: queryKeys.todo.detail(id),
          queryFn: () => apiClient.todo.getById(id),
        });
      },
    },
    };
}