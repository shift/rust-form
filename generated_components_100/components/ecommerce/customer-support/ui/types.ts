export interface customersupportConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface customersupportState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface customersupportActions {
  type: string;
  payload?: any;
}

export interface customersupportProps {
  className?: string;
  config?: customersupportConfig;
  onAction?: (action: customersupportActions) => void;
  onStateChange?: (state: customersupportState) => void;
}
