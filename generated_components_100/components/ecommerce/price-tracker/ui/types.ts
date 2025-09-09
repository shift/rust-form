export interface pricetrackerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface pricetrackerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface pricetrackerActions {
  type: string;
  payload?: any;
}

export interface pricetrackerProps {
  className?: string;
  config?: pricetrackerConfig;
  onAction?: (action: pricetrackerActions) => void;
  onStateChange?: (state: pricetrackerState) => void;
}
