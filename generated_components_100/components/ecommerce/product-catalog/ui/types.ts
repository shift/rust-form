export interface productcatalogConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface productcatalogState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface productcatalogActions {
  type: string;
  payload?: any;
}

export interface productcatalogProps {
  className?: string;
  config?: productcatalogConfig;
  onAction?: (action: productcatalogActions) => void;
  onStateChange?: (state: productcatalogState) => void;
}
