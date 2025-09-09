export interface invoicesystemConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface invoicesystemState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface invoicesystemActions {
  type: string;
  payload?: any;
}

export interface invoicesystemProps {
  className?: string;
  config?: invoicesystemConfig;
  onAction?: (action: invoicesystemActions) => void;
  onStateChange?: (state: invoicesystemState) => void;
}
