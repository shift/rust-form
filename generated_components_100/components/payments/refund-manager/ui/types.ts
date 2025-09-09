export interface refundmanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface refundmanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface refundmanagerActions {
  type: string;
  payload?: any;
}

export interface refundmanagerProps {
  className?: string;
  config?: refundmanagerConfig;
  onAction?: (action: refundmanagerActions) => void;
  onStateChange?: (state: refundmanagerState) => void;
}
