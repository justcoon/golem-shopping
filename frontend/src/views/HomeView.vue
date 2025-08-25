<template>
  <div class="home-view">
    <section class="hero">
      <div class="container">
        <div class="hero-content">
          <h1>Welcome to Golem Shopping</h1>
          <p class="lead">Discover amazing products at great prices</p>
          <router-link to="/products" class="btn btn-primary btn-lg">
            Shop Now
          </router-link>
        </div>
      </div>
    </section>

    <section class="featured-products">
      <div class="container">
        <h2 class="section-title">Featured Products</h2>
        <div v-if="isLoading" class="loading">Loading products...</div>
        <div v-else-if="error" class="error">
          Error loading products: {{ error.message }}
        </div>
        <div v-else class="product-grid">
          <div v-for="product in featuredProducts" :key="product.id" class="product-card">
            <div class="product-image">
              <img :src="getProductImage(product)" :alt="product.name" />
            </div>
            <div class="product-details">
              <h3 class="product-title">
                <router-link :to="`/products/${product.id}`">
                  {{ product.name }}
                </router-link>
              </h3>
              <p class="product-brand">{{ product.brand }}</p>
              <div class="product-price">
                <span class="sale-price">${{ getSalePrice(product) }}</span>
                <span v-if="hasDiscount(product)" class="original-price">
                  ${{ getOriginalPrice(product) }}
                </span>
              </div>
              <button 
                class="btn btn-primary btn-block mt-2"
                @click="addToCart(product)"
                :disabled="isAddingToCart"
              >
                <span v-if="isAddingToCart">Adding...</span>
                <span v-else>Add to Cart</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useProductStore } from '@/stores/productStore';
import { useCartStore } from '@/stores/cartStore';
import {Product} from "@/api/services/productService.ts";

const productStore = useProductStore();
const cartStore = useCartStore();

const isAddingToCart = ref(false);
const MOCK_USER_ID = 'user-123'; // In a real app, this would come from auth

// Fetch products when component mounts
onMounted(async () => {
  if (productStore.products.length === 0) {
    await productStore.search(''); // Empty search to get featured products
  }
});

// Get featured products (first 4 products for the homepage)
const featuredProducts = computed(() => {
  return productStore.products.slice(0, 4);
});

const isLoading = computed(() => productStore.isLoading);
const error = computed(() => productStore.error);

// Helper methods
const getProductImage = (product: any) => {
  // In a real app, you would get the image URL from the product data
  return `https://via.placeholder.com/300x200?text=${encodeURIComponent(product.name)}`;
};

const getSalePrice = (product: any) => {
  // In a real app, you would get the price from the product data
  return (product.price?.sale || product.price?.list || 0).toFixed(2);
};

const getOriginalPrice = (product: any) => {
  // In a real app, you would get the price from the product data
  return (product.price?.list || 0).toFixed(2);
};

const hasDiscount = (product: any) => {
  // Check if there's a sale price lower than the list price
  return product.price?.sale && product.price.sale < product.price?.list;
};

const addToCart = async (product: Product) => {
  try {
    isAddingToCart.value = true;
    await cartStore.addItem(MOCK_USER_ID, product["product-id"], 1);
    // Show success message or notification
  } catch (err) {
    console.error('Error adding to cart:', err);
  } finally {
    isAddingToCart.value = false;
  }
};
</script>

<style scoped>
.home-view {
  min-height: calc(100vh - 200px);
}

.hero {
  background-color: var(--color-primary);
  color: white;
  padding: 4rem 0;
  text-align: center;
  margin-bottom: 3rem;
}

.hero-content {
  max-width: 800px;
  margin: 0 auto;
}

.hero h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.lead {
  font-size: 1.25rem;
  margin-bottom: 2rem;
  opacity: 0.9;
}

.section-title {
  text-align: center;
  margin-bottom: 2rem;
  font-size: 2rem;
  color: var(--color-gray-800);
}

.product-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 2rem;
  padding: 1rem 0;
}

.product-card {
  background: white;
  border-radius: var(--border-radius);
  overflow: hidden;
  box-shadow: var(--box-shadow-sm);
  transition: var(--transition-base);
  display: flex;
  flex-direction: column;
}

.product-card:hover {
  transform: translateY(-5px);
  box-shadow: var(--box-shadow);
}

.product-image {
  width: 100%;
  height: 200px;
  overflow: hidden;
}

.product-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.product-card:hover .product-image img {
  transform: scale(1.05);
}

.product-details {
  padding: 1.25rem;
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.product-title {
  font-size: 1.1rem;
  margin-bottom: 0.5rem;
}

.product-title a {
  color: var(--color-gray-900);
  text-decoration: none;
}

.product-title a:hover {
  color: var(--color-primary);
  text-decoration: none;
}

.product-brand {
  color: var(--color-gray-600);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
}

.product-price {
  margin-top: 0.5rem;
  font-weight: 600;
  font-size: 1.25rem;
  color: var(--color-primary);
}

.sale-price {
  color: var(--color-danger);
  font-weight: bold;
  margin-right: 0.5rem;
}

.original-price {
  text-decoration: line-through;
  color: var(--color-gray-500);
  font-size: 1rem;
  font-weight: normal;
}

.loading,
.error {
  text-align: center;
  padding: 2rem;
  font-size: 1.1rem;
}

.error {
  color: var(--color-danger);
}

.btn-lg {
  padding: 0.75rem 1.5rem;
  font-size: 1.1rem;
  border-radius: var(--border-radius-lg);
}
</style>
