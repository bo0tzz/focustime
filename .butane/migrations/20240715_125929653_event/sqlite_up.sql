CREATE TABLE Event (
id INTEGER NOT NULL PRIMARY KEY,
event_type TEXT NOT NULL,
timestamp TEXT NOT NULL,
activity INTEGER,
FOREIGN KEY (activity) REFERENCES Activity (id)
);
