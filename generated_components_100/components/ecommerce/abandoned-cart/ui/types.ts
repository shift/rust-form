export interface abandonedcartConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface abandonedcartState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface abandonedcartActions {
  type: string;
  payload?: any;
}

export interface abandonedcartProps {
  className?: string;
  config?: abandonedcartConfig;
  onAction?: (action: abandonedcartActions) => void;
  onStateChange?: (state: abandonedcartState) => void;
}
