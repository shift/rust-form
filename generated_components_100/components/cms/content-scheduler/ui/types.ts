export interface contentschedulerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface contentschedulerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface contentschedulerActions {
  type: string;
  payload?: any;
}

export interface contentschedulerProps {
  className?: string;
  config?: contentschedulerConfig;
  onAction?: (action: contentschedulerActions) => void;
  onStateChange?: (state: contentschedulerState) => void;
}
