CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL DEFAULT '',
    last_name TEXT NOT NULL DEFAULT '',
    email TEXT NOT NULL DEFAULT '',
    is_admin BOOL NOT NULL DEFAULT false

);