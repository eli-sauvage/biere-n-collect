INSERT INTO Categories (id, name) VALUES (1, "biere");
INSERT INTO Categories (id, name) VALUES (2, "vin");
INSERT INTO Categories (id, name) VALUES (3, "nourriture");

INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (1, "ipa", "...ipa description", 100, 1, TRUE, 1);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (2, "blonde", "...blonde description", 50, 2, TRUE, 1);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (3, "cidre", "...cidre description", 0, 3, TRUE, 1);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (4, "vin rouge", "..vin description", 10, 1, TRUE, 2);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (5, "saucission", "..desc", 25, 1, TRUE, 3);
INSERT INTO Products(id, name, description, stock_quantity, position, available_to_order, category_id)
VALUES (6, "test to cat", "..desc", 1, 1, TRUE, NULL);
INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("pinte", 1, 820, 0.5);
INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("demie", 1, 500, 0.25);
INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("pichet", 1, 2400, 1.5);


INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("pinte", 2, 640, 0.5);

INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("demi", 3, 550, 0.25);

INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("verre", 4, 650, 0.125);

INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("", 5, 650, 1.0);

INSERT INTO ProductVariations (name, product_id, price_ht, volume)
VALUES ("", 6, 100, 1.0);

INSERT INTO Users (email, role) VALUES ("elicolh@gmail.com", "admin");
INSERT INTO Users (email, role) VALUES ("eli-sauvage@utt.fr", "waiter");