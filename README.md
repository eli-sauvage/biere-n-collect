# Biere-n-collect

🇬🇧 [English version here](README-en.md)

Ceci est une application web permettant le paiement de produits dans un bar en payant sur son téléphone.

> Besoin / cible : lors d'évènements, le goulot d'étranglement qui crée de l'attente au bar est parfois le process de paiement (manque de TPE, 1 seule caisse enregisteuse, etc.).

> **Il ne s'agit pas d'une application de service à table, le client doit aller chercher son produit au bar**

Actuellement, l'application utilise les services de Stripe pour proposer des paiements **sécurisés**.

Scénario type d'une commande :
- Le client choisit ses produits et constitue son panier
- Le client paye sa commande sur le site (via carte bleue, Apple Pay ou Google Pay)
- Un QR-code est généré et affiché au client (une copie du qr-code ainsi qu'un récapitulatif de la commande est envoyée par mail au client)
- Le client montre le QR-code à un serveur au bar
- Le serveur scanne le QR-code via la page serveur du site (authentification) et obtient le détail de la commande du client
- Le serveur peut alors servir la commande au client et marquée la commande comme "servie".


## Fonctionnement technique
### Front end

Framework [Vue.js](https://vuejs.org/), exclusivement en génération statique, pas de SSR.

3 pages principales:
- **Page client**, qui contient la liste des produits ainsi que la vue du Panier
    - **page de paiement** où une commande peut être réglée
    - **page de retour** où le client est redirigé après un paiement, c'est ici que le QR-code sera affiché.
- **Vue serveur** composée d'un scanneur de QR-code en plein écran, lorsqu'un QR-code représentant une commande est détéctée, le détail de la commande s'affiche. *(Necessite une authentification avec un compte ayant le role de "serveur" ou "admin")*
- **Vue admin** *(Necessite une authentification avec un compte ayant le rôle "admin")*
    - ouverture/fermeture du bar, configuration du message affiché à l'utilisateur lorsque le bar est fermé
    - génération de rapports d'ouverture, qui récapitule l'ensemble des produits commandés, avec le détail des prix HT et TTC
    - ajout, retrait et modification de produits (IPA, blonde, ...) et de variations (demie, pinte, pichet, ...)
    - ajout et retrait de comptes (comptes serveur ou compte admin).
### Back end
Ecrit en [Rust](https://www.rust-lang.org/), il s'agit d'une API REST classique, connectée à une base de données MariaDB (MySQL)


#### Authentification

Présence d'un système de comptes utilisateur (roles Waiter et Admin).

Aucun mot de passe stocké, il s'agit d'une authentification par OTP (One Time Password) envoyé par mail.
- l'utilisateur entre son mail
- si le compte est reconnu, un mail contenant un code à 6 chiffres est envoyé à l'adresse
- l'utilisateur doit entrer le code reçu pour créer sa session

Les sessions sont valables 12 heures, et possèdent un identifiant unique (format UUID) qui est envoyé sous forme de Cookie avec chaque requête admin/serveur.

Le server d'API regarde si l'utilisateur a le rôle suffisant pour effectuer l'opération désirée.
> exemple : un compte "serveur" ne peut pas envoyer une requête de modification d'un produit

### Paiement
L'application utilise Stripe.
- Pour le backend

Communication avec l'API de Stripe pour créer les payment intents et récupérer son status (payé, annulé, etc.).

- Pour le front end

On utilise l'intégration de stripe (crée l'élément HTML sécurisé pour sécuriser le paiement)

### Technologies utilisées

Backend
- Serveur HTTP REST : [axum](https://github.com/tokio-rs/axum)
- Connection à la base de données : [sqlx](https://github.com/launchbadge/sqlx)
- Génération du QR-code envoyé par mail en png et envoyé au client en svg : [qrcode](https://github.com/kennytm/qrcode-rust)
- Envoi de mail : [lettre](https://github.com/lettre/lettre)
- Client http pour communiquer avec l'API de Stripe : [reqwest](https://github.com/seanmonstar/reqwest)

Front end
- Scan du qr code pour la page serveur : [qr-scanner](https://github.com/nimiq/qr-scanner)
- Génération du rapport en pdf : [html2pdf](https://github.com/eKoopmans/html2pdf.js)