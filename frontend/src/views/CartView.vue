<template>
  <div class="cart">
    <h1>Your Cart</h1>
    <p v-if="!cart?.items?.length">Your cart is empty</p>
    <div v-else>
      <div v-for="item in cart.items" :key="item['product-id']" class="cart-item">
        <h3>{{ item["product-name"] }}</h3>
        <p>${{ (item.price / 100).toFixed(2) }} × {{ item.quantity }}</p>
        <button @click="updateQty(item['product-id'], item.quantity - 1)">-</button>
        <span>{{ item.quantity }}</span>
        <button @click="updateQty(item['product-id'], item.quantity + 1)">+</button>
        <button @click="removeItem(item['product-id'])">Remove</button>
      </div>
      <div class="summary">
        <h3>Total: ${{ (cart.total / 100).toFixed(2) }}</h3>
        <router-link to="/checkout" class="btn">Checkout</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useCartStore } from '@/stores/cartStore';

const cartStore = useCartStore();
const MOCK_USER_ID = 'user-123';

const cart = cartStore.cart;

onMounted(() => {
  cartStore.fetchCart(MOCK_USER_ID);
});

function updateQty(productId: string, qty: number) {
  if (qty < 1) return;
  cartStore.updateItem(MOCK_USER_ID, productId, qty);
}

function removeItem(productId: string) {
  cartStore.removeItem(MOCK_USER_ID, productId);
}
</script>

<style scoped>
.cart {
  max-width: 800px;
  margin: 0 auto;
  padding: 1rem;
}
.cart-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #eee;
}
.summary {
  margin-top: 2rem;
  text-align: right;
}
.btn {
  background: #4a6fa5;
  color: white;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  text-decoration: none;
}
</style>
