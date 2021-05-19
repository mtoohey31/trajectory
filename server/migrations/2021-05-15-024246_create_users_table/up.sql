-- Your SQL goes here
CREATE TABLE users (
  username VARCHAR NOT NULL,
  double_hashed_passwd VARCHAR NOT NULL,
  hash_salt VARCHAR NOT NULL,
  data VARCHAR,
  vault_salt VARCHAR NOT NULL,
  PRIMARY KEY (username)
);
