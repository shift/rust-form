export interface analyticsdashboardConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface analyticsdashboardState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface analyticsdashboardActions {
  type: string;
  payload?: any;
}

export interface analyticsdashboardProps {
  className?: string;
  config?: analyticsdashboardConfig;
  onAction?: (action: analyticsdashboardActions) => void;
  onStateChange?: (state: analyticsdashboardState) => void;
}
