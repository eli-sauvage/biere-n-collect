INSERT INTO ProductTypes (name, price) VALUES ("ipa", 820);
INSERT INTO ProductTypes (name, price) VALUES ("cidre", 760);
INSERT INTO ProductTypes (name, price) VALUES ("blonde", 580);
INSERT INTO ProductTypes (name, price) VALUES ("noël", 1000);

INSERT INTO Stocks (product_id, stock) VALUES(
    (SELECT id from ProductTypes WHERE name = "ipa"),
    1000
);

INSERT INTO Stocks (product_id, stock) VALUES(
    (SELECT id from ProductTypes WHERE name = "cidre"),
    200
);


INSERT INTO Stocks (product_id, stock) VALUES(
    (SELECT id from ProductTypes WHERE name = "blonde"),
    500
);


INSERT INTO Stocks (product_id, stock) VALUES(
    (SELECT id from ProductTypes WHERE name = "noël"),
    0
);