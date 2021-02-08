CREATE TABLE twoot (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES "user"(id)
);