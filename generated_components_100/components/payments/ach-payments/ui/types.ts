export interface achpaymentsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface achpaymentsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface achpaymentsActions {
  type: string;
  payload?: any;
}

export interface achpaymentsProps {
  className?: string;
  config?: achpaymentsConfig;
  onAction?: (action: achpaymentsActions) => void;
  onStateChange?: (state: achpaymentsState) => void;
}
