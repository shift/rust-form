export interface couponmanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface couponmanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface couponmanagerActions {
  type: string;
  payload?: any;
}

export interface couponmanagerProps {
  className?: string;
  config?: couponmanagerConfig;
  onAction?: (action: couponmanagerActions) => void;
  onStateChange?: (state: couponmanagerState) => void;
}
