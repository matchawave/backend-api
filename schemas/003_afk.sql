DROP TABLE IF EXISTS afk_statuses;
CREATE TABLE afk_statuses(
    user_id TEXT NOT NULL, -- User ID
    guild_id TEXT DEFAULT NULL, -- Guild ID for server-specific AFK, NULL for global AFK
    reason TEXT NOT NULL DEFAULT "", -- Reason for AFK
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')), -- When the user went AFK
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, guild_id)
);

DROP TABLE IF EXISTS afk_configs;
CREATE TABLE afk_configs(
    user_id TEXT PRIMARY KEY, -- User ID
    per_guild BOOLEAN NOT NULL DEFAULT 0, -- Whether AFK statuses will be per guild or global
    default_reason TEXT DEFAULT null,-- Message to show when user is AFK
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);