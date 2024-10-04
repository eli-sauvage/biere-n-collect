# Biere-n-collect
🇫🇷 [Version française ici](README.md)

This is a web app allowing customer to pay in a bar from their cellphone.

> Needs / target : during event, the bottleneck creating wait for customer can sometimes be the payment at the counter (missing payment terminals, only one cash register, ...)

> **This is not an app for table service, the customer still has to fetch their order at the counter**

Currently, the app uses Stripe to allow for **secured** payments.

Classic process of an order :
- The customer choses what they want to order and creates their cart
- The customer pays their order on the app (by card, Apple Pay or Google Pay)
- A QR-code is generated and displayed to the customer (a copy of the QR-code as well as a summary of the order is sent to the customer by email).
- The customer shows the QR-code to a waiter at the counter.
- The waiter scans the QR-code on the website (authentification) and get the details of the order.
- The waiter can now serve the customer and mark the order as "served".

## Technical Details
### Front end

Framework [Vue.js](https://vuejs.org/), only static built, no SSR.

3 main pages :
- **Client page**, containing the list of products as well as a view of the cart
    - **payment page** where the order can be paid
    - **return page** where the customer is redirected after a paiment, this is where the QR-code will be displayed.
- **Waiter view**, containing a QR-code scanner. When a code representing a valid order is scanned, the detail of the order are displayed *(This page needs authentification with an account having the role "admin" or "waiter")*.
- **Admin view** *(This page needs authentification with an account having the role "admin")*
    - opening/closing of the bar, configuration of the message that will be displayed to customers when the bar is closed
    - generation of reports, summarizing every order since the opening, with detail of prices (excl. tax, ...)
    - insertion, editing and deletion of products (types of beer) or variation (small, large, ...)
    - insertion and deletion of user accounts (waiter or admin)
### Back end
Written in [Rust](https://www.rust-lang.org/),this is a classical REST API, paired to a MariaDB (MySQL) database.


#### Authentification

User account system (with Admin and Waite roles).

No passwords are stored, the system only performs authentification by email OTP (One Time Password).

Auth process :
- the user inputs their email
- if the account exists, an email containing a 6-digit code is sent to the email address
- the user then has to enter the code to create their session

Sessions are valid during 12 hours, and have a unique identifier (UUID format) with takes the form of a Cookie sent alongside every admin/waiter request.

The server checks if the current user has the correct role before performing the operation.
> example: a "waiter" account cannot send a request to edit a product

### Payement
The app uses Stripe.
- Backend

Communication with Stripe API for the creation of payment intent and for retreiving their status (payed, canceled, ...)

- Front end

We use the Stripe integration (creates the HTML element for us, secured for payments)

### Used technologies

Backend
- REST HTTP server : [axum](https://github.com/tokio-rs/axum)
- Database connection : [sqlx](https://github.com/launchbadge/sqlx)
- QR-code generation, sent as png by email and as svg to the web client : [qrcode](https://github.com/kennytm/qrcode-rust)
- Sending email : [lettre](https://github.com/lettre/lettre)
- HTTP client to communicate with the Stripe API : [reqwest](https://github.com/seanmonstar/reqwest)

Frontend
- QR-code scanning for the waiter view : [qr-scanner](https://github.com/nimiq/qr-scanner)
- PDF report generation : [html2pdf](https://github.com/eKoopmans/html2pdf.js)