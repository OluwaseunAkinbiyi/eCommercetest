class eCommercePage {
    navigate() {
      cy.visit('https://www.jumia.com.ng/'); 
    }
  
    searchProduct(productName) {
        cy.get('#fi-q').type('iphone 16')
        cy.get('#search > .btn').click()
    }
  
    addToCart() {
        cy.get(':nth-child(1) > .core > .img-c > .img').click()

        cy.wait(4000)
        
        cy.get('#add-to-cart > .add').scrollIntoView().click()

        cy.get(':nth-child(1) > .-pr > [aria-label="increase cart quantity"]').click()
    }
  
    proceedToCheckout() {
        cy.get('.-mhl > ._prim').click()
    
        cy.get('.card > .-fs0 > .btn').click()
    }
  }
  
  export const eCommercePageObj = new eCommercePage();
  