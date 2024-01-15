CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    description varchar NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);