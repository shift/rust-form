export interface paymentdashboardConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface paymentdashboardState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface paymentdashboardActions {
  type: string;
  payload?: any;
}

export interface paymentdashboardProps {
  className?: string;
  config?: paymentdashboardConfig;
  onAction?: (action: paymentdashboardActions) => void;
  onStateChange?: (state: paymentdashboardState) => void;
}
