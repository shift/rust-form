export interface contentworkflowConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface contentworkflowState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface contentworkflowActions {
  type: string;
  payload?: any;
}

export interface contentworkflowProps {
  className?: string;
  config?: contentworkflowConfig;
  onAction?: (action: contentworkflowActions) => void;
  onStateChange?: (state: contentworkflowState) => void;
}
