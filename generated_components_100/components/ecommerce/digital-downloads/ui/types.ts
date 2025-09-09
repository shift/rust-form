export interface digitaldownloadsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface digitaldownloadsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface digitaldownloadsActions {
  type: string;
  payload?: any;
}

export interface digitaldownloadsProps {
  className?: string;
  config?: digitaldownloadsConfig;
  onAction?: (action: digitaldownloadsActions) => void;
  onStateChange?: (state: digitaldownloadsState) => void;
}
