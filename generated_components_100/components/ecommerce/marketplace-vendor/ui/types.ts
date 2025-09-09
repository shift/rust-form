export interface marketplacevendorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface marketplacevendorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface marketplacevendorActions {
  type: string;
  payload?: any;
}

export interface marketplacevendorProps {
  className?: string;
  config?: marketplacevendorConfig;
  onAction?: (action: marketplacevendorActions) => void;
  onStateChange?: (state: marketplacevendorState) => void;
}
