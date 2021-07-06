-- Add migration script here
CREATE TABLE IF NOT EXISTS triox_users (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(60) UNIQUE NOT NULL,
  email VARCHAR(60) UNIQUE DEFAULT NULL,
  password TEXT NOT NULL,
  locale CHAR(5),
  role SMALLINT NOT NULL DEFAULT 0,
  status VARCHAR(40)
);
