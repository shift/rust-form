export interface contenteditorConfig {
  apiEndpoint?: string;
  theme?: 'light' | 'dark';
  locale?: string;
  features?: string[];
}

export interface contenteditorState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
  data: Record<string, any>;
}

export interface contenteditorActions {
  type: string;
  payload?: any;
}

export interface contenteditorProps {
  className?: string;
  config?: contenteditorConfig;
  onAction?: (action: contenteditorActions) => void;
  onStateChange?: (state: contenteditorState) => void;
}
