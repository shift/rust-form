export interface frauddetectionConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface frauddetectionState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface frauddetectionActions {
  type: string;
  payload?: any;
}

export interface frauddetectionProps {
  className?: string;
  config?: frauddetectionConfig;
  onAction?: (action: frauddetectionActions) => void;
  onStateChange?: (state: frauddetectionState) => void;
}
