export interface phoneverificationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface phoneverificationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface phoneverificationActions {
  type: string;
  payload?: any;
}

export interface phoneverificationProps {
  className?: string;
  config?: phoneverificationConfig;
  onAction?: (action: phoneverificationActions) => void;
  onStateChange?: (state: phoneverificationState) => void;
}
