import React from 'react';
import type { Component } from '../generated/types';

interface ComponentCardProps {
  component: Component;
  onRefresh: () => void;
}

export const ComponentCard: React.FC<ComponentCardProps> = ({ component, onRefresh }) => {
  const getQualityBadgeColor = (score: number) => {
    if (score >= 80) return 'bg-green-100 text-green-800';
    if (score >= 60) return 'bg-yellow-100 text-yellow-800';
    return 'bg-red-100 text-red-800';
  };

  const parseKeywords = (keywords: any): string[] => {
    if (Array.isArray(keywords)) return keywords;
    if (typeof keywords === 'string') return [keywords];
    return [];
  };

  const getComponentCategory = (keywords: string[]): string => {
    const categories = ['auth', 'payments', 'ecommerce', 'dashboards', 'cms'];
    const found = keywords.find(k => categories.includes(k));
    return found || 'general';
  };

  const keywords = parseKeywords(component.keywords);
  const category = getComponentCategory(keywords);
  const qualityScore = 80; // Default score, could be from manifest_data

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 hover:shadow-md transition-shadow">
      <div className="p-6">
        {/* Header */}
        <div className="flex items-start justify-between mb-4">
          <div className="flex-1">
            <h3 className="text-lg font-semibold text-gray-900 mb-1">
              {component.name}
            </h3>
            <div className="flex items-center space-x-2">
              <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                category === 'auth' ? 'bg-blue-100 text-blue-800' :
                category === 'payments' ? 'bg-green-100 text-green-800' :
                category === 'ecommerce' ? 'bg-purple-100 text-purple-800' :
                category === 'dashboards' ? 'bg-orange-100 text-orange-800' :
                category === 'cms' ? 'bg-pink-100 text-pink-800' :
                'bg-gray-100 text-gray-800'
              }`}>
                {category}
              </span>
              {component.version && (
                <span className="text-sm text-gray-500">v{component.version}</span>
              )}
            </div>
          </div>
          <div className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getQualityBadgeColor(qualityScore)}`}>
            {qualityScore}/100
          </div>
        </div>

        {/* Description */}
        {component.description && (
          <p className="text-gray-600 text-sm mb-4 line-clamp-3">
            {component.description}
          </p>
        )}

        {/* Author */}
        {component.author && (
          <div className="flex items-center text-sm text-gray-500 mb-4">
            <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clipRule="evenodd" />
            </svg>
            {component.author}
          </div>
        )}

        {/* Keywords/Tags */}
        {keywords.length > 0 && (
          <div className="mb-4">
            <div className="flex flex-wrap gap-1">
              {keywords.slice(0, 3).map((keyword, index) => (
                <span
                  key={index}
                  className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800"
                >
                  {keyword}
                </span>
              ))}
              {keywords.length > 3 && (
                <span className="text-xs text-gray-500">
                  +{keywords.length - 3} more
                </span>
              )}
            </div>
          </div>
        )}

        {/* Actions */}
        <div className="flex items-center justify-between pt-4 border-t border-gray-100">
          <div className="flex items-center space-x-2">
            <button className="text-sm text-blue-600 hover:text-blue-800 font-medium">
              View Details
            </button>
            <button className="text-sm text-gray-500 hover:text-gray-700">
              Install
            </button>
          </div>
          <div className="flex items-center space-x-1">
            <button
              onClick={onRefresh}
              className="p-1 text-gray-400 hover:text-gray-600"
              title="Refresh component"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </button>
            <span className="text-xs text-gray-400">
              {new Date(component.cached_at).toLocaleDateString()}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};