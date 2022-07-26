-- Add migration script here
CREATE TABLE tasks (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name varchar(255) NOT NULL,
	due_date TIMESTAMP NOT NULL,
	postpone_count INTEGER NOT NULL DEFAULT 0
);