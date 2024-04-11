-- Add migration script here
CREATE TABLE account(
       id uuid NOT NULL,
       PRIMARY KEY (id),
       email TEXT NOT NULL UNIQUE,
       name TEXT NOT NULL,
       subscribed_on timestamptz NOT NULL
);
