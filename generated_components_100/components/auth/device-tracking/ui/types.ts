export interface devicetrackingConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface devicetrackingState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface devicetrackingActions {
  type: string;
  payload?: any;
}

export interface devicetrackingProps {
  className?: string;
  config?: devicetrackingConfig;
  onAction?: (action: devicetrackingActions) => void;
  onStateChange?: (state: devicetrackingState) => void;
}
