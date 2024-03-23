CREATE TABLE friends (
    pioki_id VARCHAR(32) NOT NULL,
    pioki_friend_id VARCHAR(32) NOT NULL,
    is_blocked BOOLEAN DEFAULT false NOT NULL,
    aka VARCHAR(32),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (pioki_id, pioki_friend_id),
    FOREIGN KEY (pioki_id) REFERENCES users(pioki_id),
    FOREIGN KEY (pioki_friend_id) REFERENCES users(pioki_id)
);

CREATE INDEX idx_friends_pioki_id ON friends (pioki_id);