export interface adminpanelConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface adminpanelState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface adminpanelActions {
  type: string;
  payload?: any;
}

export interface adminpanelProps {
  className?: string;
  config?: adminpanelConfig;
  onAction?: (action: adminpanelActions) => void;
  onStateChange?: (state: adminpanelState) => void;
}
