import { BrowserMultiFormatReader } from '@zxing/library'

beforeEach(() => {
    // TODO: Remove once https://github.com/cypress-io/cypress/issues/23772 is complete
    cy.intercept('https://r.stripe.com/0', (req) => {
        // the origin header is not getting set which is causing the request to fail
        req.headers['origin'] = 'https://js.stripe.com'
    })

    // TODO: Remove once https://github.com/cypress-io/cypress/issues/23772 is complete
    cy.intercept('https://js.stripe.com/v3', (req) => {
        req.on('response', (res) => {
            // stripe is attempting to change window.top location so change it to window.self
            res.body = res.body.replaceAll('window.top', 'window.self')
        })
    })
})
describe('template spec', () => {
    it('passes', () => {
        cy.open_bar_if_closed()
        cy.visit(Cypress.env('SERVER_URL'))

        cy.get('.add-and-price > :not(button[disabled])').as('product_count')

        cy.get('.add-and-price > :not(button[disabled])')
            .find('span.p-badge')
            .as('prices')

        cy.get('.add-and-price > :not(button[disabled])').click({
            multiple: true,
        })

        let total_parsed = 0

        cy.get('.see-cart-back > button> span.p-badge').then((total) => {
            total_parsed = parseFloat(total.text().replace('€', ''))
            let expected = 0
            cy.get('@prices')
                .each((e) => {
                    expected += parseFloat(e.text().replace('€', ''))
                })
                .then(() => {
                    expect(total_parsed).to.eq(expected)
                })
            cy.wrap(total_parsed)
        })

        cy.get('.see-cart-back > button').click()
        cy.get('@product_count').then((products) => {
            cy.get('table > tbody')
                .children()
                .should('have.length', products.length)
            cy.wrap(products.length).as('product_count')
        })

        cy.get('table > tfoot > tr > :nth-child(2) > span').then((total) => {
            expect(parseFloat(total.text().replace('€', ''))).to.eq(
                total_parsed
            )
        })

        cy.get('.cart > .button > button > span.p-badge').then((total) => {
            expect(parseFloat(total.text().replace('€', ''))).to.eq(
                total_parsed
            )
        })

        cy.get('.cart > .button > button').click()

        cy.get('.form-container > h2', { timeout: 60000 }).then((total) => {
            expect(
                parseFloat(
                    total.text().replace('Total à payer :', '').replace('€', '')
                )
            ).to.eq(total_parsed)
        })

        cy.get('button#submit > span').then((total) => {
            expect(
                parseFloat(total.text().replace('Payer', '').replace('€', ''))
            ).to.eq(total_parsed)
        })

        cy.wait(1000)
        cy.get('#link-authentication-element div iframe')
            .its('0.contentDocument.body')
            .should('not.be.empty')
            .then(cy.wrap)
            .find('input[name=email]', { timeout: 30000 })
            .should('have.value', 'elicolh@gmail.com')
            .wait(100)
            .clear()
            .type('example@example.com')
        cy.get('#payment-element div iframe')
            .its('0.contentDocument.body')
            .should('not.be.empty')
            .then(cy.wrap)
            .find('input[name=number]')
            .type('4242424242424242')
        cy.get('#payment-element div iframe')
            .its('0.contentDocument.body')
            .find('input[name=expiry]')
            .type('1234')
        cy.get('#payment-element div iframe')
            .its('0.contentDocument.body')
            .find('input[name=cvc]')
            .type('123')
        cy.get('#payment-element div iframe')
            .its('0.contentDocument.body')
            .find('select[name=country]')
            .select('France')

        cy.get('button#submit').click()

        cy.get('.container > span:nth-child(3)', { timeout: 15000 }).contains(
            'example@example.com'
        )

        cy.get('img.qr-code')
            .should('be.visible')
            .and((img) => {
                // "naturalWidth" and "naturalHeight" are set when the image loads
                expect(
                    (img[0] as HTMLImageElement).naturalWidth
                ).to.be.greaterThan(0)
            })

        cy.get('@product_count').then((prod_count) => {
            cy.get('table > tbody').children().should('have.length', prod_count)
        })
        cy.get('table > tfoot > tr > :nth-child(2) > span').then((total) => {
            expect(parseFloat(total.text().replace('€', ''))).to.eq(
                total_parsed
            )
        })

        cy.get('.qr-code').then(async (el) => {
            let image = el.get()[0] as HTMLImageElement
            const reader = new BrowserMultiFormatReader()
            console.log(image)
            let result = (await reader.decodeFromImageUrl(image.src)).getText()
            let expected = image.alt
                .match(
                    /[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/
                )
                .toString()
            expect(result).to.eq(expected)
            cy.login()

            cy.visit(Cypress.env('SERVER_URL') + '/serveur')

            cy.get('button[aria-label="Stopper le Scan"]').click()

            cy.get('input#receipt-search-order').type(result)

            cy.get('span.pi-search').parent().click()

            cy.get('.orders')
                .children()
                .first()
                .should('have.class', 'data')
                .as('command')

            cy.get('@command')
                .get('.infos')
                .children()
                .first()
                .should('have.text', 'example@example.com')

            cy.get('@command').get('.receipt').should('have.text', result)

            cy.get('@command')
                .get('.served')
                .get('i')
                .should('have.class', 'pi-times')

            cy.get('@command')
                .get('.p-tag-label')
                .should('contain.text', total_parsed + ' €TTC')

            cy.get('@command').click()

            cy.get('@product_count').then((p_count) => {
                cy.get('tbody').children().should('have.length', p_count)
            })
            cy.get('tfoot tr')
                .children()
                .last()
                .should('contain.text', total_parsed + ' €')
        })
    })
})
