<template>
  <div class="product-list-view">
    <div class="container">
      <h1>Products</h1>
      <div class="search-container">
        <input
          v-model="searchQuery"
          @input="debouncedSearch"
          placeholder="Search products..."
          class="search-input"
        />
      </div>

      <div v-if="isLoading" class="loading">Loading products...</div>
      
      <div v-else-if="error" class="error">
        Error: {{ error.message }}
        <button @click="retryLoading" class="btn btn-outline-primary">Retry</button>
      </div>

      <div v-else>
        <div class="filters">
          <select v-model="sortBy" class="form-select">
            <option value="name-asc">Name (A-Z)</option>
            <option value="name-desc">Name (Z-A)</option>
            <option value="price-asc">Price (Low to High)</option>
            <option value="price-desc">Price (High to Low)</option>
          </select>
          
          <select v-model="selectedBrand" class="form-select">
            <option value="">All Brands</option>
            <option v-for="brand in availableBrands" :key="brand" :value="brand">
              {{ brand }}
            </option>
          </select>
        </div>

        <div v-if="filteredProducts.length === 0" class="no-results">
          No products found.
          <button @click="clearFilters" class="btn btn-primary">Clear Filters</button>
        </div>

        <div v-else class="product-grid">
          <div v-for="product in paginatedProducts" :key="product.id" class="product-card">
            <div class="product-image">
              <img :src="getProductImage(product)" :alt="product.name" />
              <span v-if="isProductOnSale(product)" class="sale-badge">Sale</span>
            </div>
            <div class="product-details">
              <h3><router-link :to="`/products/${product.id}`">{{ product.name }}</router-link></h3>
              <p class="brand">{{ product.brand }}</p>
              <div class="price">
                ${{ getSalePrice(product) }}
                <span v-if="isProductOnSale(product)" class="original-price">
                  ${{ getOriginalPrice(product) }}
                </span>
              </div>
              <button 
                @click="addToCart(product)" 
                class="btn btn-primary"
                :disabled="isAddingToCart"
              >
                Add to Cart
              </button>
            </div>
          </div>
        </div>

        <div v-if="totalPages > 1" class="pagination">
          <button @click="currentPage--" :disabled="currentPage === 1">Previous</button>
          <span>Page {{ currentPage }} of {{ totalPages }}</span>
          <button @click="currentPage++" :disabled="currentPage >= totalPages">Next</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useProductStore } from '@/stores/productStore';
import { useCartStore } from '@/stores/cartStore';
import { debounce } from 'lodash-es';
import {Product} from "@/api/services/productService.ts";

const router = useRouter();
const productStore = useProductStore();
const cartStore = useCartStore();

// State
const searchQuery = ref('');
const sortBy = ref('name-asc');
const selectedBrand = ref('');
const currentPage = ref(1);
const itemsPerPage = 12;
const isAddingToCart = ref(false);
const MOCK_USER_ID = 'user-123';

// Computed
const { isLoading, error, products: allProducts } = productStore;

const availableBrands = computed(() => {
  const brands = new Set<string>();
  allProducts.forEach(p => p.brand && brands.add(p.brand));
  return Array.from(brands).sort();
});

const filteredProducts = computed(() => {
  let result = [...allProducts];
  
  // Search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(p => 
      p.name.toLowerCase().includes(query) ||
      p.brand.toLowerCase().includes(query) ||
      (p.tags && p.tags.some((t: string) => t.toLowerCase().includes(query)))
    );
  }
  
  // Brand filter
  if (selectedBrand.value) {
    result = result.filter(p => p.brand === selectedBrand.value);
  }
  
  // Sorting
  return result.sort((a, b) => {
    switch (sortBy.value) {
      case 'name-asc': return a.name.localeCompare(b.name);
      case 'name-desc': return b.name.localeCompare(a.name);
      case 'price-asc': return (a.price?.sale || a.price?.list || 0) - (b.price?.sale || b.price?.list || 0);
      case 'price-desc': return (b.price?.sale || b.price?.list || 0) - (a.price?.sale || a.price?.list || 0);
      default: return 0;
    }
  });
});

// Pagination
const totalPages = computed(() => Math.ceil(filteredProducts.value.length / itemsPerPage));
const paginatedProducts = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  return filteredProducts.value.slice(start, start + itemsPerPage);
});

// Watchers
watch([searchQuery, selectedBrand, sortBy], () => {
  currentPage.value = 1;
});

// Methods
const debouncedSearch = debounce(performSearch, 300);

async function performSearch() {
  await productStore.search(searchQuery.value || '');
}

async function retryLoading() {
  await performSearch();
}

function clearFilters() {
  searchQuery.value = '';
  selectedBrand.value = '';
  sortBy.value = 'name-asc';
  currentPage.value = 1;
}

function getProductImage(product: any) {
  return `https://via.placeholder.com/300x200?text=${encodeURIComponent(product.name)}`;
}

function getSalePrice(product: any) {
  return (product.price?.sale || product.price?.list || 0).toFixed(2);
}

function getOriginalPrice(product: any) {
  return (product.price?.list || 0).toFixed(2);
}

function isProductOnSale(product: any) {
  return product.price?.sale && product.price.sale < product.price?.list;
}

async function addToCart(product: Product) {
  try {
    isAddingToCart.value = true;
    await cartStore.addItem(MOCK_USER_ID, product["product-id"], 1);
  } catch (err) {
    console.error('Error adding to cart:', err);
  } finally {
    isAddingToCart.value = false;
  }
}

// Lifecycle
onMounted(async () => {
  if (allProducts.length === 0) {
    await productStore.search('');
  }
});
</script>

<style scoped>
.product-list-view {
  padding: 2rem 0;
}

.search-container {
  margin: 1rem 0;
  display: flex;
  gap: 0.5rem;
}

.search-input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.filters {
  display: flex;
  gap: 1rem;
  margin: 1rem 0;
}

.form-select {
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: white;
}

.product-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 2rem;
  margin: 2rem 0;
}

.product-card {
  border: 1px solid #eee;
  border-radius: 8px;
  overflow: hidden;
  transition: transform 0.2s;
}

.product-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.product-image {
  position: relative;
  height: 200px;
  background: #f8f9fa;
}

.product-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.sale-badge {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: #dc3545;
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: bold;
}

.product-details {
  padding: 1rem;
}

.product-details h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
}

.product-details h3 a {
  color: #333;
  text-decoration: none;
}

.product-details h3 a:hover {
  color: #4a6fa5;
}

.brand {
  color: #666;
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
}

.price {
  font-weight: bold;
  font-size: 1.2rem;
  margin: 0.5rem 0;
  color: #4a6fa5;
}

.original-price {
  text-decoration: line-through;
  color: #999;
  font-size: 0.9rem;
  margin-left: 0.5rem;
}

.product-details button {
  width: 100%;
  margin-top: 0.5rem;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin: 2rem 0;
}

.pagination button {
  padding: 0.5rem 1rem;
  border: 1px solid #ddd;
  background: white;
  cursor: pointer;
  border-radius: 4px;
}

.pagination button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.loading, .error, .no-results {
  text-align: center;
  padding: 2rem;
  font-size: 1.1rem;
}

.error {
  color: #dc3545;
}

.no-results {
  color: #666;
}
</style>
