CREATE TABLE Activity (
id INTEGER NOT NULL PRIMARY KEY,
class TEXT NOT NULL,
title TEXT NOT NULL
);
CREATE UNIQUE INDEX class_title_idx ON Activity(class, title);
CREATE TABLE IF NOT EXISTS butane_migrations (
name TEXT NOT NULL PRIMARY KEY
);