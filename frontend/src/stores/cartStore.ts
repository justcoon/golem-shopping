import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { 
  getCart, 
  addToCart as addToCartApi, 
  removeFromCart as removeFromCartApi, 
  updateCartEmail as updateCartEmailApi,
  updateBillingAddress as updateBillingAddressApi,
  updateShippingAddress as updateShippingAddressApi,
  checkoutCart as checkoutCartApi,
  Cart,
  CartItem
} from '@/api/services/cartService';
import { useProductStore } from './productStore';

export const useCartStore = defineStore('cart', () => {
  const cart = ref<Cart | null>(null);
  const isLoading = ref(false);
  const error = ref<Error | null>(null);
  const productStore = useProductStore();

  const cartItems = computed(() => cart.value?.items || []);
  const itemCount = computed(() => 
    cartItems.value.reduce((total, item) => total + item.quantity, 0)
  );

  const subtotal = computed(() => {
    return cartItems.value.reduce((total, item) => {
      const price = item.product?.price?.sale || item.product?.price?.list || 0;
      return total + (price * item.quantity);
    }, 0);
  });

  const fetchCart = async (userId: string) => {
    isLoading.value = true;
    error.value = null;
    
    try {
      const cartData = await getCart(userId);
      cart.value = cartData;
      
      // Fetch product details for each item in the cart
      await Promise.all(cartData.items.map(async (item: CartItem) => {
        try {
          const product = await productStore.fetchProduct(item.productId);
          item.product = product;
        } catch (err) {
          console.error(`Error fetching product ${item.productId}:`, err);
        }
      }));
      
      return cartData;
    } catch (err) {
      error.value = err as Error;
      console.error('Error fetching cart:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const addItem = async (userId: string, productId: string, quantity: number = 1) => {
    try {
      await addToCartApi(userId, productId, quantity);
      await fetchCart(userId);
    } catch (err) {
      error.value = err as Error;
      console.error('Error adding item to cart:', err);
      throw err;
    }
  };

  const removeItem = async (userId: string, productId: string) => {
    try {
      await removeFromCartApi(userId, productId);
      await fetchCart(userId);
    } catch (err) {
      error.value = err as Error;
      console.error('Error removing item from cart:', err);
      throw err;
    }
  };

  const updateEmail = async (userId: string, email: string) => {
    try {
      await updateCartEmailApi(userId, email);
      if (cart.value) {
        cart.value.email = email;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating cart email:', err);
      throw err;
    }
  };

  const updateBillingAddress = async (userId: string, address: any) => {
    try {
      await updateBillingAddressApi(userId, address);
      if (cart.value) {
        cart.value.billingAddress = address;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating billing address:', err);
      throw err;
    }
  };

  const updateShippingAddress = async (userId: string, address: any) => {
    try {
      await updateShippingAddressApi(userId, address);
      if (cart.value) {
        cart.value.shippingAddress = address;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating shipping address:', err);
      throw err;
    }
  };

  const checkout = async (userId: string) => {
    try {
      const order = await checkoutCartApi(userId);
      cart.value = null; // Clear cart after successful checkout
      return order;
    } catch (err) {
      error.value = err as Error;
      console.error('Error during checkout:', err);
      throw err;
    }
  };

  const clearCart = () => {
    cart.value = null;
  };

  return {
    cart,
    cartItems,
    itemCount,
    subtotal,
    isLoading,
    error,
    fetchCart,
    addItem,
    removeItem,
    updateEmail,
    updateBillingAddress,
    updateShippingAddress,
    checkout,
    clearCart,
  };
});
