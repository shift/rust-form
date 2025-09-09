export interface oauth2integrationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface oauth2integrationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface oauth2integrationActions {
  type: string;
  payload?: any;
}

export interface oauth2integrationProps {
  className?: string;
  config?: oauth2integrationConfig;
  onAction?: (action: oauth2integrationActions) => void;
  onStateChange?: (state: oauth2integrationState) => void;
}
