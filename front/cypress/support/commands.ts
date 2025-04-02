/// <reference types="cypress" />
// ***********************************************
// This example commands.ts shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
//
// declare global {
//   namespace Cypress {
//     interface Chainable {
//       login(email: string, password: string): Chainable<void>
//       drag(subject: string, options?: Partial<TypeOptions>): Chainable<Element>
//       dismiss(subject: string, options?: Partial<TypeOptions>): Chainable<Element>
//       visit(originalFn: CommandOriginalFn, url: string, options: Partial<VisitOptions>): Chainable<Element>
//     }
//   }
// }
//
Cypress.Commands.add('login', () => {
    cy.session('admin', () => {
        cy.request('DELETE', `${Cypress.env('MAIL_API_URL')}/api/Messages/*`)
        cy.visit(Cypress.env('SERVER_URL') + '/admin')
        cy.url().should('include', '/login')
        cy.get('input#email').type('elicolh@gmail.com')
        cy.get('button[type=submit]').contains('Valider').click()
        cy.wait(2000)
        cy.log('ok')
        cy.request(`${Cypress.env('MAIL_API_URL')}/api/Messages/new`).then(
            (response) => {
                cy.log(JSON.stringify(response))
                cy.request(
                    `${Cypress.env('MAIL_API_URL')}/api/Messages/${response.body[0].id}/plaintext`
                ).then((response) => {
                    let code = (response.body as string).match(
                        /\d\d\s*-\s*\d\d\s*-\s*\d\d/
                    )[0]
                    cy.get('.otp-container input').type(code)
                })
            }
        )

        cy.get('button[type=submit]').contains('Valider').click()
        cy.wait(1000)
    })
})

Cypress.Commands.add('open_bar_if_closed', () => {
    cy.login()
    cy.visit(Cypress.env('SERVER_URL') + '/admin')
    cy.get('button.open-close-btn', { timeout: 20000 }).should('exist')
    cy.get('body').then((b) => {
        let btn_content = b
            .find('button.open-close-btn > span:nth-child(2)')
            .text()
        if (btn_content == 'Ouvrir les commandes') {
            cy.get('button.open-close-btn').click()
            cy.get('button.p-button-success[aria-label=Ouvrir]')
                .first()
                .click({ force: true })
            cy.get('button.open-close-btn > span:nth-child(2)').should(
                'contain.text',
                'Fermer les commandes'
            )
        }
    })
})
