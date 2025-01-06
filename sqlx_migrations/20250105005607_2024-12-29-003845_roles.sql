CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO roles (name)
VALUES
    ('USER'),
    ('VIP'),
    ('ADMIN')
ON CONFLICT (name) DO NOTHING;