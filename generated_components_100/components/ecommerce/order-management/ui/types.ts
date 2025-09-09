export interface ordermanagementConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface ordermanagementState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface ordermanagementActions {
  type: string;
  payload?: any;
}

export interface ordermanagementProps {
  className?: string;
  config?: ordermanagementConfig;
  onAction?: (action: ordermanagementActions) => void;
  onStateChange?: (state: ordermanagementState) => void;
}
