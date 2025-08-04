import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { 
  getOrder, 
  getUserOrders as getUserOrdersApi, 
  updateOrderEmail as updateOrderEmailApi,
  updateOrderBillingAddress as updateOrderBillingAddressApi,
  updateOrderShippingAddress as updateOrderShippingAddressApi,
  shipOrder as shipOrderApi,
  cancelOrder as cancelOrderApi,
  Order,
  OrderItem
} from '@/api/services/orderService';
import { useProductStore } from './productStore';

export const useOrderStore = defineStore('orders', () => {
  const orders = ref<Order[]>([]);
  const currentOrder = ref<Order | null>(null);
  const isLoading = ref(false);
  const error = ref<Error | null>(null);
  const productStore = useProductStore();

  const fetchOrder = async (orderId: string) => {
    isLoading.value = true;
    error.value = null;
    
    try {
      const order = await getOrder(orderId);
      currentOrder.value = order;
      
      // Fetch product details for each item in the order
      await Promise.all(order.items.map(async (item: OrderItem) => {
        try {
          const product = await productStore.fetchProduct(item.productId);
          // In a real app, you might want to store this in a normalized state
          // For simplicity, we're attaching it directly to the item here
          (item as any).product = product;
        } catch (err) {
          console.error(`Error fetching product ${item.productId}:`, err);
        }
      }));
      
      return order;
    } catch (err) {
      error.value = err as Error;
      console.error(`Error fetching order ${orderId}:`, err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const fetchUserOrders = async (userId: string) => {
    isLoading.value = true;
    error.value = null;
    
    try {
      // Note: This would need to be implemented in the backend
      // For now, we'll return an empty array
      const userOrders = await getUserOrdersApi(userId);
      orders.value = userOrders;
      return userOrders;
    } catch (err) {
      error.value = err as Error;
      console.error(`Error fetching orders for user ${userId}:`, err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const updateEmail = async (orderId: string, email: string) => {
    try {
      await updateOrderEmailApi(orderId, email);
      if (currentOrder.value) {
        currentOrder.value.email = email;
      }
      
      // Also update in the orders list if it exists there
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex !== -1) {
        orders.value[orderIndex].email = email;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating order email:', err);
      throw err;
    }
  };

  const updateBillingAddress = async (orderId: string, address: any) => {
    try {
      await updateOrderBillingAddressApi(orderId, address);
      if (currentOrder.value) {
        currentOrder.value.billingAddress = address;
      }
      
      // Also update in the orders list if it exists there
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex !== -1) {
        orders.value[orderIndex].billingAddress = address;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating order billing address:', err);
      throw err;
    }
  };

  const updateShippingAddress = async (orderId: string, address: any) => {
    try {
      await updateOrderShippingAddressApi(orderId, address);
      if (currentOrder.value) {
        currentOrder.value.shippingAddress = address;
      }
      
      // Also update in the orders list if it exists there
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex !== -1) {
        orders.value[orderIndex].shippingAddress = address;
      }
    } catch (err) {
      error.value = err as Error;
      console.error('Error updating order shipping address:', err);
      throw err;
    }
  };

  const ship = async (orderId: string) => {
    try {
      await shipOrderApi(orderId);
      if (currentOrder.value) {
        currentOrder.value.status = 'shipped';
      }
      
      // Update status in the orders list if it exists there
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex !== -1) {
        orders.value[orderIndex].status = 'shipped';
      }
    } catch (err) {
      error.value = err as Error;
      console.error(`Error shipping order ${orderId}:`, err);
      throw err;
    }
  };

  const cancel = async (orderId: string) => {
    try {
      await cancelOrderApi(orderId);
      if (currentOrder.value) {
        currentOrder.value.status = 'cancelled';
      }
      
      // Update status in the orders list if it exists there
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex !== -1) {
        orders.value[orderIndex].status = 'cancelled';
      }
    } catch (err) {
      error.value = err as Error;
      console.error(`Error cancelling order ${orderId}:`, err);
      throw err;
    }
  };

  const clearCurrentOrder = () => {
    currentOrder.value = null;
  };

  return {
    orders,
    currentOrder,
    isLoading,
    error,
    fetchOrder,
    fetchUserOrders,
    updateEmail,
    updateBillingAddress,
    updateShippingAddress,
    ship,
    cancel,
    clearCurrentOrder,
  };
});
