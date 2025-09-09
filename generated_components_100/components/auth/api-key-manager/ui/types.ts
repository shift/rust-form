export interface apikeymanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface apikeymanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface apikeymanagerActions {
  type: string;
  payload?: any;
}

export interface apikeymanagerProps {
  className?: string;
  config?: apikeymanagerConfig;
  onAction?: (action: apikeymanagerActions) => void;
  onStateChange?: (state: apikeymanagerState) => void;
}
