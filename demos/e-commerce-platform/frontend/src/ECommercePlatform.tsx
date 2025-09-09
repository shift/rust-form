import React, { useState, useEffect } from 'react';
import { ProductCatalog } from '../../generated_components_100/components/ecommerce/product-catalog/ui/product-catalog';
import { ShoppingCart } from '../../generated_components_100/components/ecommerce/shopping-cart/ui/shopping-cart';
import { OrderManagement } from '../../generated_components_100/components/ecommerce/order-management/ui/order-management';
import { ReviewSystem } from '../../generated_components_100/components/ecommerce/review-system/ui/review-system';
import { RecommendationEngine } from '../../generated_components_100/components/ecommerce/recommendation-engine/ui/recommendation-engine';
import { JwtAuthentication } from '../../generated_components_100/components/auth/jwt-authentication/ui/jwt-authentication';
import { StripeIntegration } from '../../generated_components_100/components/payments/stripe-integration/ui/stripe-integration';
import { AnalyticsDashboard } from '../../generated_components_100/components/dashboards/analytics-dashboard/ui/analytics-dashboard';

interface Product {
  id: string;
  name: string;
  price: number;
  description: string;
  image: string;
  category: string;
  stock: number;
  rating: number;
  reviews: number;
}

interface CartItem {
  id: string;
  name: string;
  price: number;
  quantity: number;
  image: string;
}

export const ECommercePlatform: React.FC = () => {
  const [currentPage, setCurrentPage] = useState<string>('products');
  const [products, setProducts] = useState<Product[]>([]);
  const [cartItems, setCartItems] = useState<CartItem[]>([]);
  const [user, setUser] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  // Sample data
  useEffect(() => {
    setProducts([
      {
        id: '1',
        name: 'Premium Laptop',
        price: 1299.99,
        description: 'High-performance laptop for professionals',
        image: '/images/laptop.jpg',
        category: 'Electronics',
        stock: 25,
        rating: 4.8,
        reviews: 156
      },
      {
        id: '2',
        name: 'Wireless Headphones',
        price: 199.99,
        description: 'Noise-cancelling wireless headphones',
        image: '/images/headphones.jpg',
        category: 'Electronics',
        stock: 50,
        rating: 4.6,
        reviews: 89
      },
      {
        id: '3',
        name: 'Smart Watch',
        price: 299.99,
        description: 'Feature-rich smartwatch with health tracking',
        image: '/images/smartwatch.jpg',
        category: 'Wearables',
        stock: 30,
        rating: 4.7,
        reviews: 203
      }
    ]);
  }, []);

  const handleAddToCart = (product: Product) => {
    const existingItem = cartItems.find(item => item.id === product.id);
    if (existingItem) {
      setCartItems(cartItems.map(item =>
        item.id === product.id
          ? { ...item, quantity: item.quantity + 1 }
          : item
      ));
    } else {
      setCartItems([...cartItems, {
        id: product.id,
        name: product.name,
        price: product.price,
        quantity: 1,
        image: product.image
      }]);
    }
  };

  const handleUpdateQuantity = (id: string, quantity: number) => {
    if (quantity <= 0) {
      setCartItems(cartItems.filter(item => item.id !== id));
    } else {
      setCartItems(cartItems.map(item =>
        item.id === id ? { ...item, quantity } : item
      ));
    }
  };

  const handleRemoveItem = (id: string) => {
    setCartItems(cartItems.filter(item => item.id !== id));
  };

  const handleAuth = (authData: any) => {
    setUser(authData.user);
    localStorage.setItem('auth_token', authData.token);
  };

  const handleCheckout = async () => {
    setLoading(true);
    // Simulate checkout process
    await new Promise(resolve => setTimeout(resolve, 2000));
    setCartItems([]);
    setCurrentPage('orders');
    setLoading(false);
  };

  const renderCurrentPage = () => {
    switch (currentPage) {
      case 'products':
        return (
          <div className="products-page">
            <h1>Product Catalog</h1>
            <ProductCatalog
              onAction={(data) => {
                if (data.type === 'add_to_cart') {
                  handleAddToCart(data.product);
                }
              }}
              config={{ products }}
            />
            <RecommendationEngine
              config={{ userId: user?.id, products }}
              onAction={(data) => console.log('Recommendation:', data)}
            />
          </div>
        );
      
      case 'cart':
        return (
          <div className="cart-page">
            <h1>Shopping Cart</h1>
            <ShoppingCart
              items={cartItems}
              onUpdateQuantity={handleUpdateQuantity}
              onRemoveItem={handleRemoveItem}
              onCheckout={handleCheckout}
            />
          </div>
        );
      
      case 'orders':
        return (
          <div className="orders-page">
            <h1>Order Management</h1>
            <OrderManagement
              onAction={(data) => console.log('Order action:', data)}
              config={{ userId: user?.id }}
            />
          </div>
        );
      
      case 'reviews':
        return (
          <div className="reviews-page">
            <h1>Product Reviews</h1>
            <ReviewSystem
              onAction={(data) => console.log('Review action:', data)}
              config={{ productId: '1' }}
            />
          </div>
        );
      
      case 'admin':
        return (
          <div className="admin-page">
            <h1>Admin Dashboard</h1>
            <AnalyticsDashboard
              onAction={(data) => console.log('Analytics action:', data)}
              config={{ timeRange: '30d' }}
            />
          </div>
        );
      
      default:
        return <div>Page not found</div>;
    }
  };

  return (
    <div className="ecommerce-platform">
      <header className="header">
        <div className="header-content">
          <h1 className="logo">RustForm E-Commerce</h1>
          <nav className="navigation">
            <button 
              onClick={() => setCurrentPage('products')}
              className={currentPage === 'products' ? 'active' : ''}
            >
              Products
            </button>
            <button 
              onClick={() => setCurrentPage('cart')}
              className={currentPage === 'cart' ? 'active' : ''}
            >
              Cart ({cartItems.length})
            </button>
            <button 
              onClick={() => setCurrentPage('orders')}
              className={currentPage === 'orders' ? 'active' : ''}
            >
              Orders
            </button>
            <button 
              onClick={() => setCurrentPage('reviews')}
              className={currentPage === 'reviews' ? 'active' : ''}
            >
              Reviews
            </button>
            {user?.role === 'admin' && (
              <button 
                onClick={() => setCurrentPage('admin')}
                className={currentPage === 'admin' ? 'active' : ''}
              >
                Admin
              </button>
            )}
          </nav>
          <div className="auth-section">
            {user ? (
              <div className="user-info">
                <span>Welcome, {user.name}</span>
                <button onClick={() => setUser(null)}>Logout</button>
              </div>
            ) : (
              <JwtAuthentication
                onAction={handleAuth}
                config={{ redirectUrl: '/dashboard' }}
              />
            )}
          </div>
        </div>
      </header>

      <main className="main-content">
        {loading ? (
          <div className="loading">Processing...</div>
        ) : (
          renderCurrentPage()
        )}
      </main>

      <footer className="footer">
        <div className="payment-methods">
          <h3>Secure Payments</h3>
          <StripeIntegration
            onAction={(data) => console.log('Payment:', data)}
            config={{ 
              publicKey: 'pk_test_...',
              currency: 'usd'
            }}
          />
        </div>
        <div className="footer-info">
          <p>© 2024 RustForm E-Commerce. Powered by 20+ components.</p>
        </div>
      </footer>
    </div>
  );
};

export default ECommercePlatform;