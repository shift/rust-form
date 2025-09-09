export interface seooptimizerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface seooptimizerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface seooptimizerActions {
  type: string;
  payload?: any;
}

export interface seooptimizerProps {
  className?: string;
  config?: seooptimizerConfig;
  onAction?: (action: seooptimizerActions) => void;
  onStateChange?: (state: seooptimizerState) => void;
}
