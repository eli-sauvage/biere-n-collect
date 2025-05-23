/// <reference types="cypress" />

declare namespace Cypress {
    interface Chainable {
        /**
         * Custom command to select DOM element by data-cy attribute.
         * @example cy.dataCy('greeting')
         */
        login(): Chainable<Element>
        open_bar_if_closed(): Chainable<Element>
    }
}
