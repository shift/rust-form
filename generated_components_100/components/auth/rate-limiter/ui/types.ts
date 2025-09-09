export interface ratelimiterConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface ratelimiterState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface ratelimiterActions {
  type: string;
  payload?: any;
}

export interface ratelimiterProps {
  className?: string;
  config?: ratelimiterConfig;
  onAction?: (action: ratelimiterActions) => void;
  onStateChange?: (state: ratelimiterState) => void;
}
