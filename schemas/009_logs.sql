DROP TABLE IF EXISTS guild_log_configs;
CREATE TABLE guild_log_configs (
    guild_id TEXT NOT NULL,
    log_type TEXT NOT NULL, -- Type of log (e.g., message, voice, moderation)
    data TEXT NOT NULL, -- JSON data as string
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, log_type)
);

DROP TABLE IF EXISTS guild_ignore_channels;
CREATE TABLE guild_ignore_channels (
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, channel_id)
);

DROP TABLE IF EXISTS guild_ignore_users;
CREATE TABLE guild_ignore_users (
    guild_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, user_id)
);