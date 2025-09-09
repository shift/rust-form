export interface reviewsystemConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface reviewsystemState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface reviewsystemActions {
  type: string;
  payload?: any;
}

export interface reviewsystemProps {
  className?: string;
  config?: reviewsystemConfig;
  onAction?: (action: reviewsystemActions) => void;
  onStateChange?: (state: reviewsystemState) => void;
}
