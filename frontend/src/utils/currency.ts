// import { useAuthStore } from "@/stores/authStore";

const CURRENCY_SYMBOLS: Record<string, string> = {
  USD: "$",
  EUR: "€",
  GBP: "£",
  JPY: "¥",
  // Add more currency symbols as needed
};

export function formatPrice(price: string | number, currency: string): string {
  // const authStore = useAuthStore();
  // const currency = authStore.pricePreferences.currency || 'USD';
  const symbol = CURRENCY_SYMBOLS[currency] || currency;

  // Convert price to number if it's a string
  const numericPrice = typeof price === "string" ? parseFloat(price) : price;

  // Format the number with 2 decimal places
  const formattedNumber = numericPrice.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });

  return `${symbol}${formattedNumber}`;
}
