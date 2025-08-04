<template>
  <div class="checkout">
    <h1>Checkout</h1>
    
    <div v-if="!cart?.items?.length">
      <p>Your cart is empty</p>
      <router-link to="/products" class="btn">Shop Now</router-link>
    </div>
    
    <div v-else class="checkout-grid">
      <form @submit.prevent="submitOrder" class="checkout-form">
        <h2>Shipping Info</h2>
        <input v-model="shipping.name" placeholder="Full Name" required>
        <input v-model="shipping.email" type="email" placeholder="Email" required>
        <input v-model="shipping.address" placeholder="Address" required>
        <div class="form-row">
          <input v-model="shipping.city" placeholder="City" required>
          <input v-model="shipping.postalCode" placeholder="Postal Code" required>
        </div>
        
        <h2>Payment</h2>
        <input v-model="payment.card" placeholder="Card Number" required>
        <div class="form-row">
          <input v-model="payment.expiry" placeholder="MM/YY" required>
          <input v-model="payment.cvc" placeholder="CVC" type="password" required>
        </div>
        
        <button type="submit" :disabled="isSubmitting" class="btn">
          {{ isSubmitting ? 'Processing...' : 'Place Order' }}
        </button>
      </form>
      
      <div class="order-summary">
        <h2>Order Summary</h2>
        <div v-for="item in cart.items" :key="item.product_id" class="order-item">
          <span>{{ item.name }} × {{ item.quantity }}</span>
          <span>${{ (item.price * item.quantity / 100).toFixed(2) }}</span>
        </div>
        <div class="order-total">
          <span>Total</span>
          <span>${{ (cart.subtotal / 100).toFixed(2) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useCartStore } from '@/stores/cartStore';
import { useOrderStore } from '@/stores/orderStore';

const cartStore = useCartStore();
const orderStore = useOrderStore();
const router = useRouter();
const MOCK_USER_ID = 'user-123';

const cart = cartStore.cart;
const isSubmitting = ref(false);
const shipping = ref({
  name: '',
  email: '',
  address: '',
  city: '',
  postalCode: ''
});

const payment = ref({
  card: '',
  expiry: '',
  cvc: ''
});

onMounted(() => {
  if (!cart.value) cartStore.fetchCart(MOCK_USER_ID);
});

async function submitOrder() {
  if (!cart.value) return;
  
  isSubmitting.value = true;
  try {
    await orderStore.createOrder(MOCK_USER_ID, {
      shippingAddress: shipping.value,
      paymentMethod: 'card',
      items: cart.value.items.map(item => ({
        product_id: item.product_id,
        quantity: item.quantity
      }))
    });
    
    await cartStore.clearCart(MOCK_USER_ID);
    router.push(`/orders/${orderStore.currentOrder?.id}`);
  } catch (error) {
    console.error('Checkout failed:', error);
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<style scoped>
.checkout {
  max-width: 1000px;
  margin: 0 auto;
  padding: 1rem;
}

.checkout-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 2rem;
  margin-top: 2rem;
}

input {
  width: 100%;
  padding: 0.5rem;
  margin-bottom: 1rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.order-summary {
  background: #f8f9fa;
  padding: 1.5rem;
  border-radius: 8px;
  height: fit-content;
}

.order-item {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  border-bottom: 1px solid #eee;
}

.order-total {
  font-weight: bold;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 2px solid #ddd;
  display: flex;
  justify-content: space-between;
}

.btn {
  background: #4a6fa5;
  color: white;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  width: 100%;
  font-size: 1rem;
  margin-top: 1rem;
}

.btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>
