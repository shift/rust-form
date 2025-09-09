export interface paymentanalyticsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface paymentanalyticsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface paymentanalyticsActions {
  type: string;
  payload?: any;
}

export interface paymentanalyticsProps {
  className?: string;
  config?: paymentanalyticsConfig;
  onAction?: (action: paymentanalyticsActions) => void;
  onStateChange?: (state: paymentanalyticsState) => void;
}
