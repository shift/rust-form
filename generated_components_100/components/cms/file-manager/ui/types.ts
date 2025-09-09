export interface filemanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface filemanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface filemanagerActions {
  type: string;
  payload?: any;
}

export interface filemanagerProps {
  className?: string;
  config?: filemanagerConfig;
  onAction?: (action: filemanagerActions) => void;
  onStateChange?: (state: filemanagerState) => void;
}
