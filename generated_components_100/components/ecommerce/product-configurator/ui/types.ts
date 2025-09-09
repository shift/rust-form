export interface productconfiguratorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface productconfiguratorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface productconfiguratorActions {
  type: string;
  payload?: any;
}

export interface productconfiguratorProps {
  className?: string;
  config?: productconfiguratorConfig;
  onAction?: (action: productconfiguratorActions) => void;
  onStateChange?: (state: productconfiguratorState) => void;
}
