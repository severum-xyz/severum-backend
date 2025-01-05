CREATE TABLE IF NOT EXISTS user_containers (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    challenge_id INT NOT NULL,
    category_id INT NOT NULL,
    container_name UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (challenge_id) REFERENCES challenges(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
