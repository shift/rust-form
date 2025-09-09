import React, { useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { ComponentCard } from './ComponentCard';
import { CategoryFilter } from './CategoryFilter';
import { ComponentGenerator } from './ComponentGenerator';
import { SearchBar } from './SearchBar';
import { apiClient } from '../generated/api-client';
import type { Component } from '../generated/types';

interface ComponentCategory {
  name: string;
  description: string;
  total_components: number;
  available_templates: string[];
}

interface ComponentCategoriesResponse {
  categories: ComponentCategory[];
}

export const ComponentBrowser: React.FC = () => {
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [showGenerator, setShowGenerator] = useState<boolean>(false);

  // Fetch components
  const { data: components, isLoading: componentsLoading, refetch: refetchComponents } = useQuery({
    queryKey: ['components'],
    queryFn: () => apiClient.get<Component[]>('/components'),
  });

  // Fetch component categories
  const { data: categoriesResponse, isLoading: categoriesLoading } = useQuery({
    queryKey: ['component-categories'],
    queryFn: () => apiClient.get<ComponentCategoriesResponse>('/components/categories'),
  });

  const categories = categoriesResponse?.categories || [];

  // Filter components based on category and search
  const filteredComponents = useMemo(() => {
    if (!components) return [];

    return components.filter(component => {
      const matchesCategory = selectedCategory === 'all' || 
        (component.keywords && component.keywords.includes(selectedCategory));
      
      const matchesSearch = searchQuery === '' ||
        component.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (component.description && component.description.toLowerCase().includes(searchQuery.toLowerCase())) ||
        (component.author && component.author.toLowerCase().includes(searchQuery.toLowerCase()));

      return matchesCategory && matchesSearch;
    });
  }, [components, selectedCategory, searchQuery]);

  const handleComponentGenerated = () => {
    refetchComponents();
    setShowGenerator(false);
  };

  if (componentsLoading || categoriesLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Component Library</h1>
              <p className="text-sm text-gray-600">
                {filteredComponents.length} components available
              </p>
            </div>
            <button
              onClick={() => setShowGenerator(true)}
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors"
            >
              Generate New Component
            </button>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Sidebar - Filters */}
          <div className="lg:col-span-1">
            <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Filters</h2>
              
              {/* Search */}
              <SearchBar value={searchQuery} onChange={setSearchQuery} />
              
              {/* Category Filter */}
              <CategoryFilter
                categories={categories}
                selectedCategory={selectedCategory}
                onCategoryChange={setSelectedCategory}
              />
              
              {/* Quick Stats */}
              <div className="mt-6">
                <h3 className="text-sm font-medium text-gray-900 mb-3">Quick Stats</h3>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Total Components</span>
                    <span className="font-medium">{components?.length || 0}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Categories</span>
                    <span className="font-medium">{categories.length}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600">Filtered Results</span>
                    <span className="font-medium">{filteredComponents.length}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Main Content - Component Grid */}
          <div className="lg:col-span-3">
            {filteredComponents.length === 0 ? (
              <div className="text-center py-12">
                <div className="max-w-sm mx-auto">
                  <svg
                    className="mx-auto h-12 w-12 text-gray-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                    />
                  </svg>
                  <h3 className="mt-2 text-sm font-medium text-gray-900">No components found</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    Try adjusting your filters or generate a new component.
                  </p>
                  <div className="mt-6">
                    <button
                      onClick={() => setShowGenerator(true)}
                      className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                    >
                      Generate Component
                    </button>
                  </div>
                </div>
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                {filteredComponents.map((component) => (
                  <ComponentCard
                    key={component.id}
                    component={component}
                    onRefresh={refetchComponents}
                  />
                ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Component Generator Modal */}
      {showGenerator && (
        <ComponentGenerator
          onClose={() => setShowGenerator(false)}
          onComponentGenerated={handleComponentGenerated}
          categories={categories}
        />
      )}
    </div>
  );
};