import apiClient from '../config';

export interface CartItem {
  productId: string;
  quantity: number;
  product?: any; // Will be populated with product details
}

export interface Cart {
  id: string;
  userId: string;
  items: CartItem[];
  email?: string;
  billingAddress?: any;
  shippingAddress?: any;
}

export const getCart = async (userId: string): Promise<Cart> => {
  try {
    const response = await apiClient.get(`/v1/cart/${userId}`);
    return response.data;
  } catch (error) {
    console.error(`Error fetching cart for user ${userId}:`, error);
    throw error;
  }
};

export const addToCart = async (userId: string, productId: string, quantity: number = 1): Promise<void> => {
  try {
    await apiClient.put(`/v1/cart/${userId}/items/${productId}`, { quantity });
  } catch (error) {
    console.error(`Error adding item ${productId} to cart:`, error);
    throw error;
  }
};

export const removeFromCart = async (userId: string, productId: string): Promise<void> => {
  try {
    await apiClient.delete(`/v1/cart/${userId}/items/${productId}`);
  } catch (error) {
    console.error(`Error removing item ${productId} from cart:`, error);
    throw error;
  }
};

export const updateCartEmail = async (userId: string, email: string): Promise<void> => {
  try {
    await apiClient.put(`/v1/cart/${userId}/email`, { email });
  } catch (error) {
    console.error(`Error updating cart email:`, error);
    throw error;
  }
};

export const updateBillingAddress = async (userId: string, address: any): Promise<void> => {
  try {
    await apiClient.put(`/v1/cart/${userId}/billing-address`, address);
  } catch (error) {
    console.error('Error updating billing address:', error);
    throw error;
  }
};

export const updateShippingAddress = async (userId: string, address: any): Promise<void> => {
  try {
    await apiClient.put(`/v1/cart/${userId}/shipping-address`, address);
  } catch (error) {
    console.error('Error updating shipping address:', error);
    throw error;
  }
};

export const checkoutCart = async (userId: string): Promise<any> => {
  try {
    const response = await apiClient.post(`/v1/cart/${userId}/checkout`, {});
    return response.data;
  } catch (error) {
    console.error('Error during checkout:', error);
    throw error;
  }
};
