
CREATE TABLE IF NOT EXISTS accounts (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    pin_hash VARCHAR(255) NOT NULL,
    deleted_at TIMESTAMP
);

-- product_type here is:
-- 0 for hot drink
-- 1 for cold drink
CREATE TABLE IF NOT EXISTS products (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    product_type SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS purchases (
    id BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    account_id BIGINT NOT NULL REFERENCES accounts(id),
    product_id BIGINT NOT NULL REFERENCES products(id),
    quantity INT NOT NULL,
    refunded BOOLEAN NOT NULL
);
