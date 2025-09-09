-- Add up migration script here
CREATE TABLE session (
    id VARCHAR(64) PRIMARY KEY,
    account_id INT4 NOT NULL,
    expiry_time TIMESTAMPTZ NOT NULL,
    utc_create TIMESTAMPTZ NOT NULL,
    utc_modified TIMESTAMPTZ NOT NULL
);

CREATE INDEX account_id_utc_create_session_idx ON session(account_id, utc_create DESC);