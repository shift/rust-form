import React from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <h1>rustform_studio App</h1>
        <p>Frontend generated with Rust-form</p>
      </div>
    </QueryClientProvider>
  );
}

export default App;
