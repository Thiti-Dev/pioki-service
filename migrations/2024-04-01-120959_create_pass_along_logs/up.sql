CREATE TABLE keep_and_pass_along_logs (
    id SERIAL PRIMARY KEY,
    pioki_id VARCHAR(32) NOT NULL REFERENCES users(pioki_id) ON DELETE CASCADE,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    is_kept BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX idx_keep_and_pass_along_logs_pioki_id ON keep_and_pass_along_logs (pioki_id);
CREATE INDEX idx_keep_and_pass_along_logs_pioki_id_and_post_id ON keep_and_pass_along_logs (pioki_id,post_id);
CREATE INDEX idx_keep_and_pass_along_logs_pioki_id_and_post_id_and_is_kept ON keep_and_pass_along_logs (pioki_id,post_id,is_kept);