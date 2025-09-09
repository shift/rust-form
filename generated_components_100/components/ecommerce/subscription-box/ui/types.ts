export interface subscriptionboxConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface subscriptionboxState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface subscriptionboxActions {
  type: string;
  payload?: any;
}

export interface subscriptionboxProps {
  className?: string;
  config?: subscriptionboxConfig;
  onAction?: (action: subscriptionboxActions) => void;
  onStateChange?: (state: subscriptionboxState) => void;
}
