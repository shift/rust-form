export interface multilanguageConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface multilanguageState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface multilanguageActions {
  type: string;
  payload?: any;
}

export interface multilanguageProps {
  className?: string;
  config?: multilanguageConfig;
  onAction?: (action: multilanguageActions) => void;
  onStateChange?: (state: multilanguageState) => void;
}
