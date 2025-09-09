export interface shoppingcartConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface shoppingcartState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface shoppingcartActions {
  type: string;
  payload?: any;
}

export interface shoppingcartProps {
  className?: string;
  config?: shoppingcartConfig;
  onAction?: (action: shoppingcartActions) => void;
  onStateChange?: (state: shoppingcartState) => void;
}
