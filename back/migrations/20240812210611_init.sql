CREATE TABLE IF NOT EXISTS Bar
(
  is_open BOOLEAN NOT NULL DEFAULT FALSE,
  open_since TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  closing_message TEXT NOT NULL DEFAULT "le bar est fermé"
);

CREATE TRIGGER one_row_Bar
BEFORE INSERT ON Bar
WHEN (SELECT COUNT(*) FROM Bar) > 1   -- limit here
BEGIN
    SELECT RAISE(FAIL, 'ERROR Bar table is limited to one row!');
END;


CREATE TABLE IF NOT EXISTS BarOpenings
(
	id INTEGER PRIMARY KEY NOT NULL,
    begin TIMESTAMP NOT NULL,
    end TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS Products
(
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  stock_quantity FLOAT NOT NULL,
  position SMALLINT UNSIGNED UNIQUE
);


CREATE TABLE IF NOT EXISTS ProductVariations
(
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    product_id INT UNSIGNED NOT NULL,
    price_ht INT NOT NULL,
    tva FLOAT NOT NULL DEFAULT 0.2,
    volume FLOAT NOT NULL,
    available_to_order BOOLEAN NOT NULL,
    CONSTRAINT `fk_product_id`
        FOREIGN KEY (product_id) REFERENCES Products (id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS Orders
(
    id INTEGER PRIMARY KEY NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires TIMESTAMP NULL,
    payment_intent_id VARCHAR(255) NOT NULL UNIQUE,
    canceled BOOLEAN NOT NULL DEFAULT FALSE,
    client_secret VARCHAR(255) NOT NULL UNIQUE,
    payment_status VARCHAR(20) CHECK( payment_status IN ("canceled", "processing", "succeeded")),
    user_email VARCHAR(255),
    receipt VARCHAR(255) UNIQUE,
    served BOOLEAN NOT NULL DEFAULT FALSE
);


CREATE TABLE IF NOT EXISTS OrderDetails
(
	id INTEGER PRIMARY KEY NOT NULL,
    order_id INT UNSIGNED NOT NULL,
    product_id INT UNSIGNED NOT NULL, -- no constrains bc might be deleted
    item_name VARCHAR(255) NOT NULL,
    unit_price_ht INT NOT NULL,
    tva FLOAT NOT NULL,
    quantity INT UNSIGNED NOT NULL,
    variation_volume FLOAT NOT NULL,
    CONSTRAINT `fk_order_id`
        FOREIGN KEY (order_id) REFERENCES Orders (id)
        ON DELETE CASCADE
        ON UPDATE RESTRICT,
    CONSTRAINT `uq_order_id_product_id` UNIQUE (order_id, item_name)
);


CREATE TABLE IF NOT EXISTS Users
(
	id INTEGER PRIMARY KEY NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(10) CHECK( role in ("admin", "waiter")) NOT NULL
);

CREATE TABLE IF NOT EXISTS Sessions
(
	id INTEGER PRIMARY KEY NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    expires TIMESTAMP NOT NULL,
    uuid VARCHAR(36) NOT NULL,
    CONSTRAINT `fk_user_id_session`
        FOREIGN KEY (user_id) REFERENCES Users (id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);
