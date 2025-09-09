export interface remembermeConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface remembermeState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface remembermeActions {
  type: string;
  payload?: any;
}

export interface remembermeProps {
  className?: string;
  config?: remembermeConfig;
  onAction?: (action: remembermeActions) => void;
  onStateChange?: (state: remembermeState) => void;
}
