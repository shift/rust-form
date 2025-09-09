export interface wishlistmanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface wishlistmanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface wishlistmanagerActions {
  type: string;
  payload?: any;
}

export interface wishlistmanagerProps {
  className?: string;
  config?: wishlistmanagerConfig;
  onAction?: (action: wishlistmanagerActions) => void;
  onStateChange?: (state: wishlistmanagerState) => void;
}
