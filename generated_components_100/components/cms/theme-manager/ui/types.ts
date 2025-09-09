export interface thememanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface thememanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface thememanagerActions {
  type: string;
  payload?: any;
}

export interface thememanagerProps {
  className?: string;
  config?: thememanagerConfig;
  onAction?: (action: thememanagerActions) => void;
  onStateChange?: (state: thememanagerState) => void;
}
