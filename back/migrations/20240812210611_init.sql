CREATE TABLE IF NOT EXISTS Stock
(
    product_id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY NOT NULL,
    position INT UNSIGNED NOT NULL,
    price INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    quantity INT NOT NULL,
    available BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS Orders
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires TIMESTAMP,
    payment_intent_id VARCHAR(255) NOT NULL UNIQUE,
    payment_status ENUM("canceled", "processing", "succeeded"),
    user_email VARCHAR(255) NOT NULL,
    receipt VARCHAR(255)
);

-- CREATE TABLE IF NOT EXISTS Payments
-- (
--     order_id INT UNSIGNED UNIQUE NOT NULL,
--     payment_intent_id VARCHAR(255) NOT NULL UNIQUE,
--     status ENUM("canceled", "processing", "succeeded") NOT NULL,
--     CONSTRAINT `fk_orderid_payment`
--         FOREIGN KEY (order_id) REFERENCES Orders (id)
--         ON DELETE RESTRICT
--         ON UPDATE RESTRICT
-- );

CREATE TABLE IF NOT EXISTS OrderDetails
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY NOT NULL,
    order_id INT UNSIGNED NOT NULL,
    product_id INT UNSIGNED NOT NULL,
    quantity INT UNSIGNED NOT NULL,
    CONSTRAINT `fk_product_id`
        FOREIGN KEY (product_id) REFERENCES Stock (product_id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT,
    CONSTRAINT `fk_order_id`
        FOREIGN KEY (order_id) REFERENCES Orders (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT,
    CONSTRAINT `uq_order_id_product_id` UNIQUE (order_id, product_id)
);


CREATE TABLE IF NOT EXISTS Users
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role ENUM("admin", "waiter") NOT NULL
);

CREATE TABLE IF NOT EXISTS Sessions
(
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    expires TIMESTAMP NOT NULL,
    uuid VARCHAR(36) NOT NULL,
    CONSTRAINT `fk_user_id_session`
        FOREIGN KEY (user_id) REFERENCES Users (id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);