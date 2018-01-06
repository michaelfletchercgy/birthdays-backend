
CREATE TABLE users (
    user_id SERIAL,
    user_name TEXT NOT NULL,
    PRIMARY KEY (user_id)
);

ALTER TABLE birthdays ADD COLUMN user_id INTEGER;

INSERT INTO users VALUES (nextval('USERS_USER_ID_SEQ'), 'michaelfletcher');

UPDATE birthdays SET user_id = (select min(user_id) FROM users);

ALTER TABLE birthdays ALTER user_id SET NOT NULL;

ALTER TABLE birthdays ADD CONSTRAINT birthdays_user_id_fk FOREIGN KEY (user_id) REFERENCES users (user_id);