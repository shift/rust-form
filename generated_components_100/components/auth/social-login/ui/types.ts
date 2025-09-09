export interface socialloginConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface socialloginState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface socialloginActions {
  type: string;
  payload?: any;
}

export interface socialloginProps {
  className?: string;
  config?: socialloginConfig;
  onAction?: (action: socialloginActions) => void;
  onStateChange?: (state: socialloginState) => void;
}
