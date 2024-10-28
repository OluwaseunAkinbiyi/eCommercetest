E-commerce Test Automation with Cypress
This project automates the end-to-end testing of an e-commerce platform (Jumia) using Cypress and follows a Page Object Model (POM) design pattern to improve code modularity and reusability.



Project Overview
This automation project is designed to:

Navigate to the Jumia website.
Search for a specific product (e.g., "iphone 16").
Add the product to the cart and adjust the quantity.
Proceed to checkout.
Getting Started
Prerequisites
To get started, I ensure the following are installed:



Node.js (version 12 or higher)
Cypress (installed via npm)
Installation
Clone this repository:


git clone [https://github.com/yourusername/ecommerce-cypress-automation](https://github.com/OluwaseunAkinbiyi/eCommercetest).git

Navigate to the project folder:
cd ecommerce-cypress-automation

Install dependencies:
npm install



Project Structure
The main structure of this project includes:

cypress/e2e/pageObjects: Contains the Page Object Model files for different pages.

eCommercePage.js: Page object representing the Jumia website’s product search and checkout functionalities.
cypress/e2e/tests: Contains test scripts to run on the defined page objects.

Key Files
eCommercePage.js: Defines interactions for navigating, searching, adding to cart, and checking out on the e-commerce site.
ecommerceTest.spec.js: The main test file that imports the page object and runs the end-to-end scenario.
Writing and Running Tests

Test Script 
The test script imports the page object and executes each step of the workflow as follows:

javascript
import { eCommercePageObj } from '../pageObjects/eCommercePage';

describe('E-commerce Test Suite', () => {
    it('should search for a product, add it to the cart, and proceed to checkout', () => {
        eCommercePageObj.navigate();
        eCommercePageObj.searchProduct('iphone 16');
        eCommercePageObj.addToCart();
        eCommercePageObj.proceedToCheckout();
    });
});



Running Tests
To open the Cypress Test Runner, I use:

on the Terminal
npx cypress open
In the Test Runner, navigate to the test file (ecommerceTest.spec.js) to execute the test.
To run tests in headless mode:

on the Terminal
npx cypress run



Page Object Model (POM) Structure
The eCommercePage.js file encapsulates all actions related to the e-commerce page:
javascript
class eCommercePage {
    navigate() {
        cy.visit('https://www.jumia.com.ng/');
    }

    searchProduct(productName) {
        cy.get('#fi-q').type(productName);
        cy.get('#search > .btn').click();
    }

    addToCart() {
        cy.get(':nth-child(1) > .core > .img-c > .img').click();
        cy.wait(4000);
        cy.get('#add-to-cart > .add').scrollIntoView().click();
        cy.get(':nth-child(1) > .-pr > [aria-label="increase cart quantity"]').click();
    }

    proceedToCheckout() {
        cy.get('.-mhl > ._prim').click();
        cy.get('.card > .-fs0 > .btn').click();
    }
}

// Export the page object instance
export const eCommercePageObj = new eCommercePage();



Best Practices
Use Page Object Model (POM): Encapsulate each page’s interactions in a dedicated class to keep tests clean and maintainable.
Assertions: Add assertions to verify that each step succeeds.
Waits: Use Cypress’s cy.wait() cautiously; aim to rely on cy.get() with should() for better control.



Troubleshooting
Element Selectors: Update any CSS selectors if the site layout changes.
Timeouts: Increase timeout settings if network speed is an issue.
