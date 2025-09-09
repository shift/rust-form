export interface chargebackhandlerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface chargebackhandlerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface chargebackhandlerActions {
  type: string;
  payload?: any;
}

export interface chargebackhandlerProps {
  className?: string;
  config?: chargebackhandlerConfig;
  onAction?: (action: chargebackhandlerActions) => void;
  onStateChange?: (state: chargebackhandlerState) => void;
}
