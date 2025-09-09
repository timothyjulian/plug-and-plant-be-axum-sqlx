-- Add down migration script here
DROP TABLE IF EXISTS session;
DROP INDEX IF EXISTS account_id_utc_create_session_idx;