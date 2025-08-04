import apiClient from '../config';

export interface OrderItem {
  productId: string;
  quantity: number;
  price: number;
}

export interface Order {
  id: string;
  userId: string;
  items: OrderItem[];
  email: string;
  billingAddress: any;
  shippingAddress: any;
  status: 'pending' | 'shipped' | 'delivered' | 'cancelled';
  createdAt: string;
  updatedAt: string;
}

export const getOrder = async (orderId: string): Promise<Order> => {
  try {
    const response = await apiClient.get(`/v1/order/${orderId}`);
    return response.data;
  } catch (error) {
    console.error(`Error fetching order ${orderId}:`, error);
    throw error;
  }
};

export const getUserOrders = async (userId: string): Promise<Order[]> => {
  try {
    // Note: This endpoint might need to be implemented in the backend
    // For now, we'll return an empty array
    return [];
  } catch (error) {
    console.error(`Error fetching orders for user ${userId}:`, error);
    throw error;
  }
};

export const updateOrderEmail = async (orderId: string, email: string): Promise<void> => {
  try {
    await apiClient.put(`/v1/order/${orderId}/email`, { email });
  } catch (error) {
    console.error(`Error updating order email:`, error);
    throw error;
  }
};

export const updateOrderBillingAddress = async (orderId: string, address: any): Promise<void> => {
  try {
    await apiClient.put(`/v1/order/${orderId}/billing-address`, address);
  } catch (error) {
    console.error('Error updating order billing address:', error);
    throw error;
  }
};

export const updateOrderShippingAddress = async (orderId: string, address: any): Promise<void> => {
  try {
    await apiClient.put(`/v1/order/${orderId}/shipping-address`, address);
  } catch (error) {
    console.error('Error updating order shipping address:', error);
    throw error;
  }
};

export const shipOrder = async (orderId: string): Promise<void> => {
  try {
    await apiClient.post(`/v1/order/${orderId}/ship-order`, {});
  } catch (error) {
    console.error(`Error shipping order ${orderId}:`, error);
    throw error;
  }
};

export const cancelOrder = async (orderId: string): Promise<void> => {
  try {
    await apiClient.post(`/v1/order/${orderId}/cancel-order`, {});
  } catch (error) {
    console.error(`Error cancelling order ${orderId}:`, error);
    throw error;
  }
};
