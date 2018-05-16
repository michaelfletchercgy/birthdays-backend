
CREATE TABLE subscriptions (
    subscription_id SERIAL,
    user_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY (subscription_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);