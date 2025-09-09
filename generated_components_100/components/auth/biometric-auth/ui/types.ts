export interface biometricauthConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface biometricauthState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface biometricauthActions {
  type: string;
  payload?: any;
}

export interface biometricauthProps {
  className?: string;
  config?: biometricauthConfig;
  onAction?: (action: biometricauthActions) => void;
  onStateChange?: (state: biometricauthState) => void;
}
