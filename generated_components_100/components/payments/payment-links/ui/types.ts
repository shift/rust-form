export interface paymentlinksConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface paymentlinksState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface paymentlinksActions {
  type: string;
  payload?: any;
}

export interface paymentlinksProps {
  className?: string;
  config?: paymentlinksConfig;
  onAction?: (action: paymentlinksActions) => void;
  onStateChange?: (state: paymentlinksState) => void;
}
