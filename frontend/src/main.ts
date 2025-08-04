import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createRouter, createWebHistory } from 'vue-router';
import App from './App.vue';
import './style.css';

// Import views
import HomeView from './views/HomeView.vue';
import ProductListView from './views/ProductListView.vue';
import ProductDetailView from './views/ProductDetailView.vue';
import CartView from './views/CartView.vue';
import CheckoutView from './views/CheckoutView.vue';
import OrderListView from './views/OrderListView.vue';
import OrderDetailView from './views/OrderDetailView.vue';
import NotFoundView from './views/NotFoundView.vue';

// Initialize Pinia
const pinia = createPinia();

// Set up router
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/products',
      name: 'products',
      component: ProductListView,
    },
    {
      path: '/products/:id',
      name: 'product-detail',
      component: ProductDetailView,
      props: true,
    },
    {
      path: '/cart',
      name: 'cart',
      component: CartView,
    },
    {
      path: '/checkout',
      name: 'checkout',
      component: CheckoutView,
    },
    {
      path: '/orders',
      name: 'orders',
      component: OrderListView,
    },
    {
      path: '/orders/:id',
      name: 'order-detail',
      component: OrderDetailView,
      props: true,
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: NotFoundView,
    },
  ],
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    } else {
      return { top: 0 };
    }
  },
});

// Create and mount the app
const app = createApp(App);

app.use(pinia);
app.use(router);

app.mount('#app');
