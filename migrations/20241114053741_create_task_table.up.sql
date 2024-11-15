-- Add up migration script here
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT CHECK(status IN ('pending', 'in_progress', 'completed')) DEFAULT 'pending',
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    date_updated TIMESTAMP
);

CREATE TRIGGER set_date_updated
AFTER UPDATE ON tasks
FOR EACH ROW
BEGIN
    UPDATE tasks SET date_updated = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
