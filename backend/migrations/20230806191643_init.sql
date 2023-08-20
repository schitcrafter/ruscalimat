
CREATE TABLE IF NOT EXISTS accounts (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    external_id VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    pin_hash VARCHAR(255),
    picture VARCHAR(255),
    deleted_at TIMESTAMP,
    balance BIGINT NOT NULL DEFAULT 0,
    has_pin BOOL NOT NULL DEFAULT FALSE
);

CREATE TYPE product_type AS ENUM ('colddrink', 'hotdrink');

CREATE TABLE IF NOT EXISTS products (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    product_type product_type NOT NULL
);

CREATE TABLE IF NOT EXISTS purchases (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    account_id BIGINT NOT NULL REFERENCES accounts(id),
    product_id BIGINT NOT NULL REFERENCES products(id),
    quantity INT NOT NULL DEFAULT 1,
    refunded BOOLEAN NOT NULL DEFAULT FALSE,
    paid_price: BIGINT NOT NULL
);
