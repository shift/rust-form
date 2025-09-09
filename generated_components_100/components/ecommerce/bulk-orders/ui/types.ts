export interface bulkordersConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface bulkordersState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface bulkordersActions {
  type: string;
  payload?: any;
}

export interface bulkordersProps {
  className?: string;
  config?: bulkordersConfig;
  onAction?: (action: bulkordersActions) => void;
  onStateChange?: (state: bulkordersState) => void;
}
