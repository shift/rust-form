export interface recommendationengineConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface recommendationengineState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface recommendationengineActions {
  type: string;
  payload?: any;
}

export interface recommendationengineProps {
  className?: string;
  config?: recommendationengineConfig;
  onAction?: (action: recommendationengineActions) => void;
  onStateChange?: (state: recommendationengineState) => void;
}
