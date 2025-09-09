export interface paymentgatewayConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface paymentgatewayState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface paymentgatewayActions {
  type: string;
  payload?: any;
}

export interface paymentgatewayProps {
  className?: string;
  config?: paymentgatewayConfig;
  onAction?: (action: paymentgatewayActions) => void;
  onStateChange?: (state: paymentgatewayState) => void;
}
