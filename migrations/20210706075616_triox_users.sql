-- Add migration script here
CREATE TABLE IF NOT EXISTS triox_users (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(60) UNIQUE NOT NULL,
  email VARCHAR(60) UNIQUE DEFAULT NULL,
  password CHAR(100) NOT NULL,
  locale CHAR(5),
  role SMALLINT NOT NULL,
  status VARCHAR(40)
);
