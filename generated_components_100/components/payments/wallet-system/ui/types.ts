export interface walletsystemConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface walletsystemState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface walletsystemActions {
  type: string;
  payload?: any;
}

export interface walletsystemProps {
  className?: string;
  config?: walletsystemConfig;
  onAction?: (action: walletsystemActions) => void;
  onStateChange?: (state: walletsystemState) => void;
}
