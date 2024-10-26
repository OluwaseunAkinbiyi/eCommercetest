describe('E-commerce Automation Test', () => {
  it('should search for a product, add to cart, and proceed to checkout', () => {
    // Visit the Jumia website
    cy.visit('https://www.jumia.com.ng/');



    // Search for the product
    cy.get('#fi-q').type('iphone 16');
    cy.get('#search > .btn').click();

    // Assert that search results contain "iphone 16"
    cy.get('.-fh').should('contain', 'iphone 16');




    // addToCart
    cy.get(':nth-child(1) > .core > .img-c > .img').click()

        cy.wait(4000)
        
        cy.get('#add-to-cart > .add').scrollIntoView().click()

        cy.get(':nth-child(1) > .-pr > [aria-label="increase cart quantity"]').click()

        // Assertion for cart item count
        cy.get(':nth-child(1) > .-pr > .-w-32')
      .should('contain', '1');





      // proceedToCheckout
        cy.get('.-mhl > ._prim').click()
    
        cy.get('.card > .-fs0 > .btn').click()

        // Assertion for human verification
        cy.get('#fbMan1 > :nth-child(1) > div')
       
    

  });
});