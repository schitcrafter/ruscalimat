
CREATE TABLE accounts (
    id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    pin_hash VARCHAR(255),
    picture VARCHAR(255),
    deleted_at TIMESTAMP,
    balance BIGINT NOT NULL DEFAULT 0
);

CREATE TYPE product_type AS ENUM ('colddrink', 'hotdrink');

CREATE TABLE products (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    product_type product_type NOT NULL,
    price BIGINT NOT NULL
);

CREATE TABLE purchases (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    account_id VARCHAR(255) NOT NULL REFERENCES accounts(id),
    product_id BIGINT NOT NULL REFERENCES products(id),
    quantity INT NOT NULL DEFAULT 1,
    refunded BOOLEAN NOT NULL DEFAULT FALSE,
    paid_price BIGINT NOT NULL
);

CREATE TABLE favorites (
    account_id VARCHAR(255) NOT NULL REFERENCES accounts(id),
    product_id BIGINT NOT NULL REFERENCES products(id),
    PRIMARY KEY(account_id, product_id)
);
