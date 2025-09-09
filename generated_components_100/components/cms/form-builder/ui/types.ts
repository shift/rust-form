export interface formbuilderConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface formbuilderState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface formbuilderActions {
  type: string;
  payload?: any;
}

export interface formbuilderProps {
  className?: string;
  config?: formbuilderConfig;
  onAction?: (action: formbuilderActions) => void;
  onStateChange?: (state: formbuilderState) => void;
}
