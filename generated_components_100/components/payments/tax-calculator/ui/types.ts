export interface taxcalculatorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface taxcalculatorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface taxcalculatorActions {
  type: string;
  payload?: any;
}

export interface taxcalculatorProps {
  className?: string;
  config?: taxcalculatorConfig;
  onAction?: (action: taxcalculatorActions) => void;
  onStateChange?: (state: taxcalculatorState) => void;
}
