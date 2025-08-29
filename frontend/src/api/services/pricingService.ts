import apiClient from "../config";
import { DateTime, dateTimeToDate } from "@/types/datetime.ts";

export interface PricingItem {
  price: number;
  currency: string;
  zone: string;
}

export interface SalePricingItem extends PricingItem {
  start?: DateTime;
  end?: DateTime;
}

export interface Pricing {
  "product-id": string;
  "msrp-prices": PricingItem[];
  "list-prices": PricingItem[];
  "sale-prices": SalePricingItem[];
  "updated-at": DateTime;
}

export const getProductPricing = async (
  productId: string,
): Promise<Pricing> => {
  try {
    const response = await apiClient.get(`/v1/pricing/${productId}`);
    return response.ok;
  } catch (error) {
    console.error(`Error fetching pricing for product ${productId}:`, error);
    throw error;
  }
};

export const getBatchPricing = async (
  productIds: string[],
): Promise<Record<string, Pricing>> => {
  try {
    // Make parallel requests for each product's pricing
    const pricingPromises = productIds.map(async (id) => {
      try {
        const pricing = await getProductPricing(id);
        return { id, pricing };
      } catch (error) {
        console.error(`Error fetching pricing for product ${id}:`, error);
        return { id, error };
      }
    });

    // Wait for all requests to complete
    const results = await Promise.all(pricingPromises);

    // Convert array of results to a record
    return results.reduce<Record<string, Pricing>>(
      (acc, { id, pricing, error }) => {
        if (pricing) {
          acc[id] = pricing;
        }
        // Skip products that had errors
        return acc;
      },
      {},
    );
  } catch (error) {
    console.error("Error in batch pricing operation:", error);
    throw error;
  }
};

export const getCurrentSalePrices = (pricing: Pricing): SalePricingItem[] => {
  const now = new Date();
  return pricing["sale-prices"].filter((sale) => {
    const start = sale.start ? dateTimeToDate(sale.start) : null;
    const end = sale.end ? dateTimeToDate(sale.end) : null;

    return (!start || now >= start) && (!end || now <= end);
  });
};

export const getBestPrice = (pricing: Pricing): number | null => {
  const salePrices = getCurrentSalePrices(pricing);
  const listPrices = pricing["list-prices"];

  if (salePrices.length > 0) {
    // Get the lowest sale price
    return Math.min(...salePrices.map((p) => p.price));
  } else if (listPrices.length > 0) {
    // Get the lowest list price if no active sales
    return Math.min(...listPrices.map((p) => p.price));
  }

  return null;
};
