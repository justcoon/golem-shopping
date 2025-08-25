import apiClient from '../config';

export interface Product {
  'product-id': string;
  name: string;
  brand: string;
  description: string;
  tags: string[];
  price?: {
    msrp: number;
    list: number;
    sale: number;
  };
}

export const searchProducts = async (query: string): Promise<Product[]> => {
  try {
    const response = await apiClient.get(`/v1/product/search?query=${encodeURIComponent(query)}`);
    return response.ok;
  } catch (error) {
    console.error('Error searching products:', error);
    throw error;
  }
};

export const getProductById = async (productId: string): Promise<Product> => {
  try {
    const response = await apiClient.get(`/v1/product/${productId}`);
    return response.ok;
  } catch (error) {
    console.error(`Error fetching product ${productId}:`, error);
    throw error;
  }
};

export const getProductPrice = async (productId: string) => {
  try {
    const response = await apiClient.get(`/v1/pricing/${productId}`);
    return response.ok;
  } catch (error) {
    console.error(`Error fetching price for product ${productId}:`, error);
    throw error;
  }
};
