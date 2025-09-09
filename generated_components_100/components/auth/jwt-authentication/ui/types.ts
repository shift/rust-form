export interface jwtauthenticationConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface jwtauthenticationState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface jwtauthenticationActions {
  type: string;
  payload?: any;
}

export interface jwtauthenticationProps {
  className?: string;
  config?: jwtauthenticationConfig;
  onAction?: (action: jwtauthenticationActions) => void;
  onStateChange?: (state: jwtauthenticationState) => void;
}
