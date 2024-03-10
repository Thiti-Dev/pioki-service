CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    pioki_id VARCHAR(255) UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Adding indexes
CREATE INDEX idx_pioki_id ON users (pioki_id);
CREATE INDEX idx_id ON users (id);