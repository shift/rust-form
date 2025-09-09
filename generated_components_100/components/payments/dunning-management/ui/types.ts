export interface dunningmanagementConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface dunningmanagementState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface dunningmanagementActions {
  type: string;
  payload?: any;
}

export interface dunningmanagementProps {
  className?: string;
  config?: dunningmanagementConfig;
  onAction?: (action: dunningmanagementActions) => void;
  onStateChange?: (state: dunningmanagementState) => void;
}
