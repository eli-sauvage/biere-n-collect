INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (1, "ipa", "...ipa description", 100, 1, TRUE);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (2, "blonde", "...blonde description", 50, 2, TRUE);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (3, "cidre", "...cidre description", 0, 3, TRUE);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (4, "vin rouge", "..vin description", 10, 1, TRUE);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (5, "saucission", "..desc", 25, 1, TRUE);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order)
VALUES (6, "test to cat", "..desc", 1, 1, TRUE);


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

INSERT INTO ProductVariations (name, product_id, price_ht, volume, available_to_order)
VALUES ("", 6, 100, 1.0, TRUE);

INSERT INTO Users (email, role) VALUES ("elicolh@gmail.com", "admin");
INSERT INTO Users (email, role) VALUES ("eli-sauvage@utt.fr", "waiter");
