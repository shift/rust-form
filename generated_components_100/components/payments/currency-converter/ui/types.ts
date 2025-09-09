export interface currencyconverterConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface currencyconverterState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface currencyconverterActions {
  type: string;
  payload?: any;
}

export interface currencyconverterProps {
  className?: string;
  config?: currencyconverterConfig;
  onAction?: (action: currencyconverterActions) => void;
  onStateChange?: (state: currencyconverterState) => void;
}
