export interface sitemapgeneratorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface sitemapgeneratorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface sitemapgeneratorActions {
  type: string;
  payload?: any;
}

export interface sitemapgeneratorProps {
  className?: string;
  config?: sitemapgeneratorConfig;
  onAction?: (action: sitemapgeneratorActions) => void;
  onStateChange?: (state: sitemapgeneratorState) => void;
}
