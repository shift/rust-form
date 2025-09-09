export interface emailverificationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface emailverificationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface emailverificationActions {
  type: string;
  payload?: any;
}

export interface emailverificationProps {
  className?: string;
  config?: emailverificationConfig;
  onAction?: (action: emailverificationActions) => void;
  onStateChange?: (state: emailverificationState) => void;
}
