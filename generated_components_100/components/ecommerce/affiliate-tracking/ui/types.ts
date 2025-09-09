export interface affiliatetrackingConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface affiliatetrackingState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface affiliatetrackingActions {
  type: string;
  payload?: any;
}

export interface affiliatetrackingProps {
  className?: string;
  config?: affiliatetrackingConfig;
  onAction?: (action: affiliatetrackingActions) => void;
  onStateChange?: (state: affiliatetrackingState) => void;
}
