-- Your SQL goes here
CREATE TABLE users (
  username VARCHAR NOT NULL,
  double_hashed_passwd VARCHAR NOT NULL,
  hash_salt BYTEA NOT NULL,
  data BYTEA,
  vault_salt BYTEA NOT NULL,
  PRIMARY KEY (username)
);
