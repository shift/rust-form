import React from 'react';

interface CategoryFilterProps {
  categories: Array<{
    name: string;
    description: string;
    total_components: number;
  }>;
  selectedCategory: string;
  onCategoryChange: (category: string) => void;
}

export const CategoryFilter: React.FC<CategoryFilterProps> = ({
  categories,
  selectedCategory,
  onCategoryChange,
}) => {
  return (
    <div className="mb-6">
      <h3 className="text-sm font-medium text-gray-900 mb-3">Categories</h3>
      <div className="space-y-2">
        <button
          onClick={() => onCategoryChange('all')}
          className={`w-full text-left px-3 py-2 rounded-md text-sm ${
            selectedCategory === 'all'
              ? 'bg-blue-100 text-blue-800 font-medium'
              : 'text-gray-600 hover:bg-gray-100'
          }`}
        >
          All Components
        </button>
        {categories.map((category) => (
          <button
            key={category.name}
            onClick={() => onCategoryChange(category.name)}
            className={`w-full text-left px-3 py-2 rounded-md text-sm ${
              selectedCategory === category.name
                ? 'bg-blue-100 text-blue-800 font-medium'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
          >
            <div className="flex justify-between items-center">
              <span className="capitalize">{category.name}</span>
              <span className="text-xs bg-gray-200 px-2 py-0.5 rounded-full">
                {category.total_components}
              </span>
            </div>
          </button>
        ))}
      </div>
    </div>
  );
};