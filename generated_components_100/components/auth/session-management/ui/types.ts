export interface sessionmanagementConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface sessionmanagementState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface sessionmanagementActions {
  type: string;
  payload?: any;
}

export interface sessionmanagementProps {
  className?: string;
  config?: sessionmanagementConfig;
  onAction?: (action: sessionmanagementActions) => void;
  onStateChange?: (state: sessionmanagementState) => void;
}
