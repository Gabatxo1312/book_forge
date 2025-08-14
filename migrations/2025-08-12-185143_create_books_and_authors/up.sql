-- Your SQL goes here

CREATE TABLE books (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  owner_id INTEGER NOT NULL,
  current_holder_id INTEGER,
  FOREIGN KEY (owner_id) REFERENCES users(id),
  FOREIGN KEY (current_holder_id) REFERENCES users(id)
);

CREATE TABLE authors (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL
);

CREATE TABLE books_authors (
  book_id INTEGER NOT NULL,
  author_id INTEGER NOT NULL,
  PRIMARY KEY (book_id, author_id),
  FOREIGN KEY (book_id) REFERENCES books(id),
  FOREIGN KEY (author_id) REFERENCES authors(id)
);
