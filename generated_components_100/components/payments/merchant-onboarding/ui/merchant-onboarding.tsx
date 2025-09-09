import React, { useState, useEffect } from 'react';

interface merchantonboardingProps {
  className?: string;
  onAction?: (data: any) => void;
  config?: Record<string, any>;
}

export const merchantonboarding: React.FC<merchantonboardingProps> = ({ 
  className = 'merchant-onboarding', 
  onAction,
  config = {}
}) => {
  const [state, setState] = useState<any>({});
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    console.log('Component merchant-onboarding initialized');
  }, []);

  const handleAction = async (actionData: any) => {
    setLoading(true);
    setError(null);
    
    try {
      if (onAction) {
        await onAction(actionData);
      }
      setState(prev => ({ ...prev, ...actionData }));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  if (error) {
    return (
      <div className={"${className}__error"}>
        <p>Error: {error}</p>
        <button onClick={() => setError(null)}>Retry</button>
      </div>
    );
  }

  return (
    <div className={className}>
      <div className={"${className}__header"}>
        <h2>merchant onboarding</h2>
      </div>
      
      <div className={"${className}__content"}>
        {loading ? (
          <div className={"${className}__loading"}>Loading...</div>
        ) : (
          <div className={"${className}__main"}>
            <p>Component content for merchant-onboarding</p>
            <button onClick={() => handleAction({ type: 'test' })}>
              Test Action
            </button>
          </div>
        )}
      </div>
      
      <div className={"${className}__footer"}>
        <small>Powered by rust-form</small>
      </div>
    </div>
  );
};

export default merchantonboarding;
