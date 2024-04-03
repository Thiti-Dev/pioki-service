CREATE TABLE friends (
    pioki_id VARCHAR(32) NOT NULL REFERENCES users(pioki_id),
    pioki_friend_id VARCHAR(32) NOT NULL REFERENCES users(pioki_id),
    is_blocked BOOLEAN DEFAULT false NOT NULL,
    aka VARCHAR(32),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (pioki_id, pioki_friend_id)
);

CREATE INDEX idx_friends_pioki_id ON friends (pioki_id);