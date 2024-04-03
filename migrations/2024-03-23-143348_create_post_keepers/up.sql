-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE post_keepers (
    id SERIAL PRIMARY KEY,
    pioki_id VARCHAR(32) NOT NULL REFERENCES users(pioki_id),
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    pass_along_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_unique_pioki_id_with_post_id ON post_keepers (pioki_id,post_id);
-- CREATE UNIQUE INDEX idx_unique_pioki_id_with_post_id_and_pass_along_at_is_null ON post_keepers (pioki_id,post_id) WHERE pass_along_at IS NULL;

-- CREATE UNIQUE INDEX idx_unique_pioki_id_with_post_id_and_pass_along_at_is_not_null ON post_keepers (pioki_id,post_id) WHERE pass_along_at IS NOT NULL;