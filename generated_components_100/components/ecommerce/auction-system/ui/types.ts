export interface auctionsystemConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface auctionsystemState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface auctionsystemActions {
  type: string;
  payload?: any;
}

export interface auctionsystemProps {
  className?: string;
  config?: auctionsystemConfig;
  onAction?: (action: auctionsystemActions) => void;
  onStateChange?: (state: auctionsystemState) => void;
}
