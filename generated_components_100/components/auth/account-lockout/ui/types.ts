export interface accountlockoutConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface accountlockoutState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface accountlockoutActions {
  type: string;
  payload?: any;
}

export interface accountlockoutProps {
  className?: string;
  config?: accountlockoutConfig;
  onAction?: (action: accountlockoutActions) => void;
  onStateChange?: (state: accountlockoutState) => void;
}
