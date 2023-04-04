CREATE TABLE wines (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    year INT NOT NULL,
    price INT NOT NULL
)