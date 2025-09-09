export interface captchaintegrationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface captchaintegrationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface captchaintegrationActions {
  type: string;
  payload?: any;
}

export interface captchaintegrationProps {
  className?: string;
  config?: captchaintegrationConfig;
  onAction?: (action: captchaintegrationActions) => void;
  onStateChange?: (state: captchaintegrationState) => void;
}
