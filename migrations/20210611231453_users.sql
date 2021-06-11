-- Add migration script here
CREATE TABLE users (
    id SERIAL,
    email varchar(50) NOT NULL,
    password varchar(128) NOT NULL
);