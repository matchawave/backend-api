DROP TABLE IF EXISTS guild_settings;
CREATE TABLE guild_settings (
    guild_id TEXT PRIMARY KEY,
    prefix TEXT DEFAULT '!' NOT NULL,
    language TEXT DEFAULT 'en' NOT NULL,
    colour TEXT DEFAULT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS command_aliases;
CREATE TABLE command_aliases(
    guild_id TEXT NOT NULL,
    command TEXT NOT NULL,
    alias TEXT NOT NULL,
    args TEXT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, command, alias)
);

