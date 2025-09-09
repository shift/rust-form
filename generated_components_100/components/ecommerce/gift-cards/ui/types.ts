export interface giftcardsConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface giftcardsState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface giftcardsActions {
  type: string;
  payload?: any;
}

export interface giftcardsProps {
  className?: string;
  config?: giftcardsConfig;
  onAction?: (action: giftcardsActions) => void;
  onStateChange?: (state: giftcardsState) => void;
}
