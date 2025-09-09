export interface mediagalleryConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface mediagalleryState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface mediagalleryActions {
  type: string;
  payload?: any;
}

export interface mediagalleryProps {
  className?: string;
  config?: mediagalleryConfig;
  onAction?: (action: mediagalleryActions) => void;
  onStateChange?: (state: mediagalleryState) => void;
}
