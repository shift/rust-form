export interface inventorytrackerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface inventorytrackerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface inventorytrackerActions {
  type: string;
  payload?: any;
}

export interface inventorytrackerProps {
  className?: string;
  config?: inventorytrackerConfig;
  onAction?: (action: inventorytrackerActions) => void;
  onStateChange?: (state: inventorytrackerState) => void;
}
