DROP TABLE IF EXISTS reaction_triggers;
CREATE TABLE reaction_triggers (
    guild_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    trigger TEXT NOT NULL,
    owner_id TEXT NOT NULL, -- User ID of the person who created the trigger
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, trigger, emoji)
);

DROP TABLE IF EXISTS reaction_channels;
CREATE TABLE reaction_channels (
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, channel_id, emoji)
);