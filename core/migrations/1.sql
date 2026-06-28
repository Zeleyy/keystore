CREATE TABLE categories (
    category_id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT UNIQUE NOT NULL,
    category_icon TEXT,
    created_at INTEGER NOT NULL DEFAULT (UNIXEPOCH())
);

CREATE TABLE entries (
    entry_id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_title TEXT NOT NULL,
    entry_url TEXT,
    entry_username TEXT,
    entry_password TEXT NOT NULL,
    entry_password_nonce TEXT NOT NULL DEFAULT '',
    entry_notes TEXT,
    category_id INTEGER REFERENCES categories(category_id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (UNIXEPOCH())
);

CREATE TABLE tags (
    tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
    tag_name TEXT UNIQUE NOT NULL
);

CREATE TABLE entry_tags (
    entry_id INTEGER REFERENCES entries(entry_id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags(tag_id) ON DELETE CASCADE,
    PRIMARY KEY (entry_id, tag_id)
);

CREATE TABLE password_history (
    password_history_id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL REFERENCES entries(entry_id) ON DELETE CASCADE,
    password TEXT NOT NULL,
    password_nonce TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL DEFAULT (UNIXEPOCH())
);

CREATE INDEX idx_entries_title ON entries(entry_title);
CREATE INDEX idx_entries_url ON entries(entry_url);
CREATE INDEX idx_entries_category ON entries(category_id);
CREATE INDEX idx_entry_tags_tag ON entry_tags(tag_id);
CREATE INDEX idx_password_history_entry ON password_history(entry_id);