export interface paypalintegrationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface paypalintegrationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface paypalintegrationActions {
  type: string;
  payload?: any;
}

export interface paypalintegrationProps {
  className?: string;
  config?: paypalintegrationConfig;
  onAction?: (action: paypalintegrationActions) => void;
  onStateChange?: (state: paypalintegrationState) => void;
}
