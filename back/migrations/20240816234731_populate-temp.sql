INSERT INTO Products(id, name, description, stock_quantity, position)
VALUES (1, "ipa", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat", 100, 1);
INSERT INTO Products(id, name, description, stock_quantity, position)
VALUES (2, "blonde", "description blonde", 50, 2);
INSERT INTO Products(id, name, description, stock_quantity, position)
VALUES (3, "cidre", "description cidre", 0, 3);
INSERT INTO Products(id, name, description, stock_quantity, position)
VALUES (4, "vin rouge", "description vin", 10, 4);
INSERT INTO Products(id, name, description, stock_quantity, position)
VALUES (5, "saucission", "description saucission", 25, 5);


INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("pinte", 1, 820, 0.5, TRUE);
INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("demie", 1, 500, 0.25, TRUE);
INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("pichet", 1, 2400, 1.5, TRUE);


INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("pinte", 2, 640, 0.5, TRUE);

INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("demi", 3, 550, 0.25, TRUE);

INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("verre", 4, 650, 0.125, TRUE);

INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("", 5, 650, 1.0, TRUE);

INSERT INTO Users (email, role) VALUES ("elicolh@gmail.com", "admin");
INSERT INTO Users (email, role) VALUES ("sauvagemartial@yahoo.fr", "admin");
INSERT INTO Users (email, role) VALUES ("eli.sauvage@utt.fr", "waiter");
