# lhavrais-pay

## RoadMap
- gestion stock -> commande annulée ou confirmée
- restucture rust codebase
    - host static frontend on rocket ?
- waiter panel
    - qr code generation/validation
- add product types
    - gestion des pintes/demies
    - gestion images de produits
- max order amount
- mobile -> prevent zoom on double click
- resume cart on return home from checkout page
- error handling on frontend !
- keep user email in browser cache ?

- switch to axum ...

# todo
Nouvelle table => Payments
    foreign key order_id
    payment_intent_id
    status
voir #[sqlx(default)] pour le futur champ Option<Payment> de Order -> Default = None
    ou pas -> requiert de faire un let mut Order = query_as!(Order, ...) puis if let some payment then Order.payement = payment ...