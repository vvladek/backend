CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

INSERT INTO users (name, email, password_hash)
VALUES 
('Alice', 'alice@example.com', 'hash1')
ON CONFLICT DO NOTHING;

INSERT INTO users (name, email, password_hash)
VALUES 
('Bob', 'bob@example.com', 'hash2')
ON CONFLICT DO NOTHING;