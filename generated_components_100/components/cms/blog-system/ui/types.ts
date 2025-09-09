export interface blogsystemConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface blogsystemState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface blogsystemActions {
  type: string;
  payload?: any;
}

export interface blogsystemProps {
  className?: string;
  config?: blogsystemConfig;
  onAction?: (action: blogsystemActions) => void;
  onStateChange?: (state: blogsystemState) => void;
}
