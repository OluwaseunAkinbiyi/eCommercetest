// Import the eCommercePage object
import { eCommercePageObj } from "./PageObject/eCommercePage";

describe('E-commerce Test Suite', () => {
    it('should search for a product, add it to the cart, and proceed to checkout', () => {
        // Navigate to the website
        eCommercePageObj.navigate();

        // Search for a specific product
        eCommercePageObj.searchProduct('iphone 16');

        // Add the product to the cart
        eCommercePageObj.addToCart();

        // Proceed to checkout
        eCommercePageObj.proceedToCheckout();
    });
});
