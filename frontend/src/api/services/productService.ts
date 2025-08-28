import apiClient from "../config";
import {
  getProductPricing,
  getBatchPricing,
  getBestPrice,
  type Pricing,
  SalePricingItem,
  PricingItem,
} from "./pricingService";
import { dateTimeToDate } from "@/types/datetime.ts";

export interface Product {
  "product-id": string;
  name: string;
  brand: string;
  description: string;
  tags: string[];
  pricing?: Pricing;
  bestPrice?: number;
}

const enhanceWithPricing = async (product: Product): Promise<Product> => {
  try {
    const pricing = await getProductPricing(product["product-id"]);
    return {
      ...product,
      pricing,
      bestPrice: getBestPrice(pricing),
    };
  } catch (error) {
    console.error(
      `Error enhancing product ${product["product-id"]} with pricing:`,
      error,
    );
    return product; // Return product without pricing if there's an error
  }
};

export const searchProducts = async (query: string): Promise<Product[]> => {
  try {
    const response = await apiClient.get(
      `/v1/product/search?query=${encodeURIComponent(query)}`,
    );
    const products: Product[] = response.ok;

    // Get pricing for all products in batch
    const productIds = products.map((p) => p["product-id"]);
    const pricingMap = await getBatchPricing(productIds);

    // Merge products with their pricing
    return products.map((product) => {
      const pricing = pricingMap[product["product-id"]];
      return {
        ...product,
        pricing,
        bestPrice: pricing ? getBestPrice(pricing) : undefined,
      };
    });
  } catch (error) {
    console.error("Error searching products:", error);
    throw error;
  }
};

export const getProductById = async (
  productId: string,
  includePricing = true,
): Promise<Product> => {
  try {
    const response = await apiClient.get(`/v1/product/${productId}`);
    const product: Product = response.ok;

    if (includePricing) {
      return enhanceWithPricing(product);
    }

    return product;
  } catch (error) {
    console.error(`Error fetching product ${productId}:`, error);
    throw error;
  }
};

// Helper to get a product with pricing (alias for backward compatibility)
export const getProductWithPricing = getProductById;

// Get multiple products by IDs with their pricing
export const getProductsByIds = async (
  productIds: string[],
): Promise<Record<string, Product>> => {
  try {
    // First get all products
    const productsResponse = await Promise.all(
      productIds.map((id) => getProductById(id, false)),
    );

    // Then get all pricing in a single batch request
    const pricingMap = await getBatchPricing(productIds);

    // Merge products with their pricing
    const result: Record<string, Product> = {};
    productsResponse.forEach((product) => {
      const pricing = pricingMap[product["product-id"]];
      result[product["product-id"]] = {
        ...product,
        pricing,
        bestPrice: pricing ? getBestPrice(pricing) : undefined,
      };
    });

    return result;
  } catch (error) {
    console.error("Error fetching products by IDs:", error);
    throw error;
  }
};

export const getProductBestPrice = (product: Product): string => {
  return product.bestPrice?.toFixed(2) || "0.00";
};

export const getProductOriginalPrice = (product: Product): string => {
  if (product.pricing && product.pricing?.["list-prices"].length > 0) {
    const minListPrice = Math.min(
      ...product.pricing["list-prices"].map((p: PricingItem) => p.price),
    );
    return minListPrice.toFixed(2);
  }
  return getProductBestPrice(product);
};

export const isProductOnSale = (product: Product): boolean => {
  if (!product.pricing?.["sale-prices"]?.length) return false;

  const bestSalePrice = Math.min(
    ...product.pricing["sale-prices"]
      .filter((sale: SalePricingItem) => {
        const now = new Date();
        const start = sale.start ? dateTimeToDate(sale.start) : null;
        const end = sale.end ? dateTimeToDate(sale.end) : null;
        return (!start || now >= start) && (!end || now <= end);
      })
      .map((s: SalePricingItem) => s.price),
  );

  const minListPrice = product.pricing?.["list-prices"]?.length
    ? Math.min(
        ...product.pricing["list-prices"].map((p: PricingItem) => p.price),
      )
    : Infinity;

  return bestSalePrice < minListPrice;
};

export const getProductImage = (product: { name: string }): string => {
  // Generate a consistent hash from the product name for deterministic image selection
  const nameHash = product.name.split("").reduce((acc, char) => {
    return char.charCodeAt(0) + ((acc << 5) - acc);
  }, 0);

  // Use picsum.photos with the hash to get a consistent but varied image per product
  const imageId = Math.abs(nameHash) % 1000;
  return `https://picsum.photos/seed/${imageId}/300/200`;
};
