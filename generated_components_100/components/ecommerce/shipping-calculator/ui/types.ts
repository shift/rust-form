export interface shippingcalculatorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface shippingcalculatorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface shippingcalculatorActions {
  type: string;
  payload?: any;
}

export interface shippingcalculatorProps {
  className?: string;
  config?: shippingcalculatorConfig;
  onAction?: (action: shippingcalculatorActions) => void;
  onStateChange?: (state: shippingcalculatorState) => void;
}
