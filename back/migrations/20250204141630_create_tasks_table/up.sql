CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0 CHECK (completed IN (0, 1))
);

INSERT INTO
    tasks (description)
VALUES
    ("demo task");

INSERT INTO
    tasks (description)
VALUES
    ("demo task2");
