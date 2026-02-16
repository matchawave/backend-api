DROP TABLE IF EXISTS embeds;
CREATE TABLE embeds (
    name TEXT NOT NULL, -- Name of the embed config, e.g., "welcome", "goodbye", "announcement"
    guild_id TEXT NOT NULL, -- Guild ID for server-specific embed
    content TEXT DEFAULT NULL,
    embeds TEXT DEFAULT "[]", -- JSON array as string
    components TEXT DEFAULT "[]", -- JSON data as string
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
    PRIMARY KEY (guild_id, name)
);