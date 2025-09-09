export interface passwordpolicyConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface passwordpolicyState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface passwordpolicyActions {
  type: string;
  payload?: any;
}

export interface passwordpolicyProps {
  className?: string;
  config?: passwordpolicyConfig;
  onAction?: (action: passwordpolicyActions) => void;
  onStateChange?: (state: passwordpolicyState) => void;
}
