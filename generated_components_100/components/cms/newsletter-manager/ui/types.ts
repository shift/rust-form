export interface newslettermanagerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface newslettermanagerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface newslettermanagerActions {
  type: string;
  payload?: any;
}

export interface newslettermanagerProps {
  className?: string;
  config?: newslettermanagerConfig;
  onAction?: (action: newslettermanagerActions) => void;
  onStateChange?: (state: newslettermanagerState) => void;
}
