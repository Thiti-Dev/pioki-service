-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    creator_id VARCHAR(32) NOT NULL REFERENCES users(pioki_id),
    spoiler_header VARCHAR(50),
    origin_quota_limit integer NOT NULL,
    quota_left integer NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);