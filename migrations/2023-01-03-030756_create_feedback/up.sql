CREATE TABLE feedbacks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  category TEXT NOT NULL,
  email TEXT,
  message TEXT NOT NULL,
  version_name TEXT,
  platform INTEGER,
  platform_version TEXT,
  branch TEXT
);

CREATE TABLE exceptions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  feedback_id INTEGER NOT NULL REFERENCES feedbacks(id),
  stack_trace TEXT NOT NULL,
  stack_trace_hash TEXT NOT NULL
);
