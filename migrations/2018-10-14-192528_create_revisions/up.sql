CREATE TABLE revisions (
  id SERIAL PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES documents(id),
  created TIMESTAMP NOT NULL,
  contents TEXT NOT NULL
)
