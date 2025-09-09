export interface rbacengineConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface rbacengineState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface rbacengineActions {
  type: string;
  payload?: any;
}

export interface rbacengineProps {
  className?: string;
  config?: rbacengineConfig;
  onAction?: (action: rbacengineActions) => void;
  onStateChange?: (state: rbacengineState) => void;
}
