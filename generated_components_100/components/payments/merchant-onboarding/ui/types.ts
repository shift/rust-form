export interface merchantonboardingConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface merchantonboardingState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface merchantonboardingActions {
  type: string;
  payload?: any;
}

export interface merchantonboardingProps {
  className?: string;
  config?: merchantonboardingConfig;
  onAction?: (action: merchantonboardingActions) => void;
  onStateChange?: (state: merchantonboardingState) => void;
}
