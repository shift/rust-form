export interface splitpaymentsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface splitpaymentsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface splitpaymentsActions {
  type: string;
  payload?: any;
}

export interface splitpaymentsProps {
  className?: string;
  config?: splitpaymentsConfig;
  onAction?: (action: splitpaymentsActions) => void;
  onStateChange?: (state: splitpaymentsState) => void;
}
