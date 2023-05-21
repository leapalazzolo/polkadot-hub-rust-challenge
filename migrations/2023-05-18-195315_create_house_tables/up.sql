-- Your SQL goes here
CREATE TABLE houses (
  id INTEGER PRIMARY KEY,
  street VARCHAR NOT NULL,
  street_number INTEGER NOT NULL,
  street_floor VARCHAR NOT NULL,
  postal_code VARCHAR NOT NULL,
  surface_square_meters INTEGER NOT NULL,
  bathrooms INTEGER NOT NULL,
  rooms INTEGER NOT NULL,
  kind_id  INTEGER NOT NULL,
  FOREIGN KEY(kind_id) REFERENCES houses_kind(id)

);

CREATE TABLE houses_kind (
  id    INTEGER PRIMARY KEY, 
  kind  VARCHAR NOT NULL
);