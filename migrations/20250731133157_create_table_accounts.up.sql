-- Add up migration script here
CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    utc_create TIMESTAMPTZ NOT NULL,
    utc_modified TIMESTAMPTZ NOT NULL
);