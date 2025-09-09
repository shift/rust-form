export interface subscriptionbillingConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface subscriptionbillingState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface subscriptionbillingActions {
  type: string;
  payload?: any;
}

export interface subscriptionbillingProps {
  className?: string;
  config?: subscriptionbillingConfig;
  onAction?: (action: subscriptionbillingActions) => void;
  onStateChange?: (state: subscriptionbillingState) => void;
}
