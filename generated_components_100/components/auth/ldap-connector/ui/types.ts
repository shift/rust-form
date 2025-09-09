export interface ldapconnectorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface ldapconnectorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface ldapconnectorActions {
  type: string;
  payload?: any;
}

export interface ldapconnectorProps {
  className?: string;
  config?: ldapconnectorConfig;
  onAction?: (action: ldapconnectorActions) => void;
  onStateChange?: (state: ldapconnectorState) => void;
}
