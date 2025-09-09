import React from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ComponentBrowser } from './components/ComponentBrowser';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <ComponentBrowser />
      </div>
    </QueryClientProvider>
  );
}

export default App;
