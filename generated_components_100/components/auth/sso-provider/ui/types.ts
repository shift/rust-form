export interface ssoproviderConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface ssoproviderState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface ssoproviderActions {
  type: string;
  payload?: any;
}

export interface ssoproviderProps {
  className?: string;
  config?: ssoproviderConfig;
  onAction?: (action: ssoproviderActions) => void;
  onStateChange?: (state: ssoproviderState) => void;
}
