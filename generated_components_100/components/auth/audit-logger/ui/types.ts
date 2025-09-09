export interface auditloggerConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface auditloggerState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface auditloggerActions {
  type: string;
  payload?: any;
}

export interface auditloggerProps {
  className?: string;
  config?: auditloggerConfig;
  onAction?: (action: auditloggerActions) => void;
  onStateChange?: (state: auditloggerState) => void;
}
