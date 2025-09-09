export interface samlintegrationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface samlintegrationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface samlintegrationActions {
  type: string;
  payload?: any;
}

export interface samlintegrationProps {
  className?: string;
  config?: samlintegrationConfig;
  onAction?: (action: samlintegrationActions) => void;
  onStateChange?: (state: samlintegrationState) => void;
}
