<template>
  <div class="order-detail">
    <div v-if="isLoading" class="loading">
      Loading order details...
    </div>
    
    <div v-else-if="error" class="error">
      Error loading order: {{ error.message }}
      <button @click="fetchOrder" class="btn btn-outline">
        Try Again
      </button>
    </div>
    
    <div v-else-if="!order" class="not-found">
      <h2>Order not found</h2>
      <p>We couldn't find the order you're looking for.</p>
      <router-link to="/orders" class="btn">
        View All Orders
      </router-link>
    </div>
    
    <div v-else class="order-container">
      <div class="order-header">
        <div>
          <h1>Order #{{ order.id }}</h1>
          <p class="order-date">Placed on {{ formatDate(order.created_at) }}</p>
        </div>
        <div class="order-status" :class="getStatusClass(order.status)">
          {{ formatStatus(order.status) }}
        </div>
      </div>
      
      <div class="order-grid">
        <div class="order-items">
          <h2>Order Items</h2>
          <div v-for="item in order.items" :key="item['product-id']" class="order-item">
            <img :src="getProductImage(item)" :alt="item['product-name']" class="item-image" />
            <div class="item-details">
              <h3>{{ item['product-name'] }}</h3>
              <p class="item-brand">{{ item['product-brand'] }}</p>
              <p class="item-price">${{ (item.price / 100).toFixed(2) }} × {{ item.quantity }}</p>
            </div>
            <div class="item-total">
              ${{ ((item.price * item.quantity) / 100).toFixed(2) }}
            </div>
          </div>
          
          <div class="order-summary">
            <div class="summary-row">
              <span>Subtotal</span>
              <span>${{ (order.total / 100).toFixed(2) }}</span>
            </div>
            <div class="summary-row">
              <span>Shipping</span>
              <span>{{ order.shipping_cost > 0 ? `$${(order.shipping_cost / 100).toFixed(2)}` : 'Free' }}</span>
            </div>
            <div class="summary-row total">
              <span>Total</span>
              <span>${{ (order.total / 100).toFixed(2) }}</span>
            </div>
          </div>
        </div>
        
        <div class="order-info">
          <div class="shipping-address">
            <h3>Shipping Address</h3>
            <template v-if="order.shipping_address">
              <p>{{ order.shipping_address.name }}</p>
              <p>{{ order.shipping_address.address }}</p>
              <p>{{ order.shipping_address.city }}, {{ order.shipping_address.state }} {{ order.shipping_address.postal_code }}</p>
              <p>{{ order.shipping_address.country }}</p>
            </template>
            <p v-else>No shipping address provided</p>
          </div>
          
          <div class="payment-info">
            <h3>Payment Method</h3>
            <p v-if="order.payment_method === 'card'">
              Credit Card ending in •••• {{ order.payment_details?.last4 || '1234' }}
            </p>
            <p v-else>{{ order.payment_method || 'No payment method specified' }}</p>
          </div>
          
          <div class="order-actions" v-if="order.status === 'PROCESSING'">
            <button @click="cancelOrder" class="btn btn-outline">
              Cancel Order
            </button>
          </div>
        </div>
      </div>
      
      <div class="order-timeline">
        <h2>Order Status</h2>
        <div class="timeline">
          <div 
            v-for="(status, index) in orderStatuses" 
            :key="status.value"
            class="timeline-step"
            :class="{
              'active': isStatusActive(status.value),
              'completed': isStatusCompleted(status.value)
            }"
          >
            <div class="timeline-marker"></div>
            <div class="timeline-content">
              <h4>{{ status.label }}</h4>
              <p v-if="getStatusDate(status.value)">
                {{ formatDate(getStatusDate(status.value)!) }}
              </p>
              <p v-else>Pending</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useOrderStore } from '@/stores/orderStore';

const route = useRoute();
const router = useRouter();
const orderStore = useOrderStore();
const MOCK_USER_ID = 'user-123';

const order = computed(() => orderStore.currentOrder);
const isLoading = computed(() => orderStore.isLoading);
const error = computed(() => orderStore.error);

const orderStatuses = [
  { value: 'PROCESSING', label: 'Order Placed' },
  { value: 'SHIPPED', label: 'Shipped' },
  { value: 'DELIVERED', label: 'Delivered' }
];

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

function formatStatus(status: string) {
  return status.split('_').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()
  ).join(' ');
}

function getStatusClass(status: string) {
  return {
    'status-processing': status === 'PROCESSING',
    'status-shipped': status === 'SHIPPED',
    'status-delivered': status === 'DELIVERED',
    'status-cancelled': status === 'CANCELLED'
  };
}

function getProductImage(item: any) {
  return `https://via.placeholder.com/80?text=${encodeURIComponent(item.name)}`;
}

function isStatusActive(status: string) {
  if (!order.value) return false;
  return order.value.status === status;
}

function isStatusCompleted(status: string) {
  if (!order.value) return false;
  const statusOrder = ['PROCESSING', 'SHIPPED', 'DELIVERED'];
  const currentStatusIndex = statusOrder.indexOf(order.value.status);
  const statusIndex = statusOrder.indexOf(status);
  return currentStatusIndex >= statusIndex;
}

function getStatusDate(status: string) {
  if (!order.value) return null;
  
  switch (status) {
    case 'PROCESSING':
      return order.value.created_at;
    case 'SHIPPED':
      return order.value.shipped_at;
    case 'DELIVERED':
      return order.value.delivered_at;
    default:
      return null;
  }
}

async function fetchOrder() {
  const orderId = route.params.id as string;
  if (orderId) {
    await orderStore.fetchOrder(MOCK_USER_ID, orderId);
  }
}

async function cancelOrder() {
  if (!order.value) return;
  
  if (confirm('Are you sure you want to cancel this order?')) {
    try {
      await orderStore.cancelOrder(MOCK_USER_ID, order.value.id);
      // Refresh the order details
      await fetchOrder();
    } catch (err) {
      console.error('Failed to cancel order:', err);
    }
  }
}

onMounted(fetchOrder);

// Watch for route changes to load the correct order
watch(() => route.params.id, fetchOrder);
</script>

<style scoped>
.order-detail {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem 1rem;
}

.loading, .error, .not-found {
  text-align: center;
  padding: 4rem 1rem;
}

.error {
  color: #dc3545;
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid #e9ecef;
}

.order-date {
  color: #6c757d;
  font-size: 1rem;
  margin-top: 0.5rem;
}

.order-status {
  padding: 0.5rem 1rem;
  border-radius: 50px;
  font-size: 0.9rem;
  font-weight: 500;
  text-transform: capitalize;
}

.status-processing {
  background: #fff3cd;
  color: #856404;
}

.status-shipped {
  background: #cce5ff;
  color: #004085;
}

.status-delivered {
  background: #d4edda;
  color: #155724;
}

.status-cancelled {
  background: #f8d7da;
  color: #721c24;
}

.order-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 2rem;
  margin-bottom: 3rem;
}

.order-items {
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 1.5rem;
}

.order-item {
  display: flex;
  gap: 1.5rem;
  padding: 1.5rem 0;
  border-bottom: 1px solid #f1f3f5;
}

.order-item:last-child {
  border-bottom: none;
}

.item-image {
  width: 80px;
  height: 80px;
  object-fit: cover;
  border-radius: 4px;
  border: 1px solid #e9ecef;
}

.item-details {
  flex: 1;
}

.item-details h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
}

.item-brand {
  color: #6c757d;
  font-size: 0.9rem;
  margin: 0 0 0.5rem 0;
}

.item-price {
  color: #495057;
  font-size: 0.95rem;
  margin: 0;
}

.item-total {
  font-weight: 500;
  font-size: 1.1rem;
}

.order-summary {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e9ecef;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.75rem;
  font-size: 0.95rem;
}

.summary-row.total {
  font-size: 1.1rem;
  font-weight: 600;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid #e9ecef;
}

.order-info > div {
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
}

.order-info h3 {
  margin-top: 0;
  margin-bottom: 1rem;
  font-size: 1.1rem;
  color: #343a40;
}

.order-info p {
  margin: 0.5rem 0;
  color: #495057;
  line-height: 1.5;
}

.order-actions {
  text-align: right;
  padding-top: 1rem;
  border-top: 1px solid #e9ecef;
  margin-top: 1.5rem;
}

.order-timeline {
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 1.5rem;
  margin-top: 2rem;
}

.timeline {
  position: relative;
  padding-left: 2rem;
  margin-top: 1.5rem;
}

.timeline::before {
  content: '';
  position: absolute;
  left: 0.5rem;
  top: 0;
  bottom: 0;
  width: 2px;
  background: #e9ecef;
}

.timeline-step {
  position: relative;
  padding-bottom: 2rem;
  padding-left: 2rem;
}

.timeline-step:last-child {
  padding-bottom: 0;
}

.timeline-marker {
  position: absolute;
  left: -1.5rem;
  top: 0.25rem;
  width: 1rem;
  height: 1rem;
  border-radius: 50%;
  background: #e9ecef;
  border: 3px solid white;
  z-index: 1;
}

.timeline-step.completed .timeline-marker {
  background: #4a6fa5;
  border-color: white;
}

.timeline-step.active .timeline-marker {
  background: white;
  border: 3px solid #4a6fa5;
}

.timeline-content h4 {
  margin: 0 0 0.25rem 0;
  font-size: 1rem;
  color: #343a40;
}

.timeline-content p {
  margin: 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.btn {
  display: inline-block;
  padding: 0.5rem 1.25rem;
  border-radius: 4px;
  font-size: 0.95rem;
  font-weight: 500;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-outline {
  background: white;
  border: 1px solid #4a6fa5;
  color: #4a6fa5;
}

.btn-outline:hover {
  background: #f8f9fa;
}

@media (max-width: 768px) {
  .order-grid {
    grid-template-columns: 1fr;
  }
  
  .order-header {
    flex-direction: column;
    gap: 1rem;
  }
  
  .order-status {
    align-self: flex-start;
  }
}
</style>
