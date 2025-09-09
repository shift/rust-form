export interface cryptocurrencywalletConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface cryptocurrencywalletState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface cryptocurrencywalletActions {
  type: string;
  payload?: any;
}

export interface cryptocurrencywalletProps {
  className?: string;
  config?: cryptocurrencywalletConfig;
  onAction?: (action: cryptocurrencywalletActions) => void;
  onStateChange?: (state: cryptocurrencywalletState) => void;
}
