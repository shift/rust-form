export interface loyaltypointsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface loyaltypointsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface loyaltypointsActions {
  type: string;
  payload?: any;
}

export interface loyaltypointsProps {
  className?: string;
  config?: loyaltypointsConfig;
  onAction?: (action: loyaltypointsActions) => void;
  onStateChange?: (state: loyaltypointsState) => void;
}
