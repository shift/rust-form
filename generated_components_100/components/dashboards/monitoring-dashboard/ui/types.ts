export interface monitoringdashboardConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface monitoringdashboardState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface monitoringdashboardActions {
  type: string;
  payload?: any;
}

export interface monitoringdashboardProps {
  className?: string;
  config?: monitoringdashboardConfig;
  onAction?: (action: monitoringdashboardActions) => void;
  onStateChange?: (state: monitoringdashboardState) => void;
}
