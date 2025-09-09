export interface multifactorauthConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface multifactorauthState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface multifactorauthActions {
  type: string;
  payload?: any;
}

export interface multifactorauthProps {
  className?: string;
  config?: multifactorauthConfig;
  onAction?: (action: multifactorauthActions) => void;
  onStateChange?: (state: multifactorauthState) => void;
}
