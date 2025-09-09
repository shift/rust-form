export interface stripeintegrationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface stripeintegrationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface stripeintegrationActions {
  type: string;
  payload?: any;
}

export interface stripeintegrationProps {
  className?: string;
  config?: stripeintegrationConfig;
  onAction?: (action: stripeintegrationActions) => void;
  onStateChange?: (state: stripeintegrationState) => void;
}
