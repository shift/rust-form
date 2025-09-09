export interface contentversioningConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface contentversioningState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface contentversioningActions {
  type: string;
  payload?: any;
}

export interface contentversioningProps {
  className?: string;
  config?: contentversioningConfig;
  onAction?: (action: contentversioningActions) => void;
  onStateChange?: (state: contentversioningState) => void;
}
