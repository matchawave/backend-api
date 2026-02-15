-- Guild Configuration Tables --
DROP TABLE IF EXISTS guild_settings;
CREATE TABLE guild_settings (
    guild_id TEXT PRIMARY KEY,
    prefix TEXT DEFAULT '!' NOT NULL,
    language TEXT DEFAULT 'en' NOT NULL,
    colour TEXT DEFAULT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

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

DROP TABLE IF EXISTS voice_configs;
CREATE TABLE voice_configs(
    user_id TEXT DEFAULT NULL, -- NULL for guild configs
    guild_id TEXT DEFAULT NULL, -- NULL for non-guild config

    name TEXT DEFAULT NULL,
    bitrate INTEGER DEFAULT NULL,
    user_limit INTEGER DEFAULT NULL,
    locked TEXT DEFAULT NULL,

    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),

    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, guild_id)
);

DROP TABLE IF EXISTS voice_masters;
CREATE TABLE voice_masters(
    guild_id TEXT PRIMARY KEY, -- Server ID
    masters TEXT NOT NULL, -- Channel ID for voice masters
    configs TEXT NOT NULL, -- Channel ID for voice configs

    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
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

DROP TABLE IF EXISTS permissions;
CREATE TABLE permissions(
    guild_id TEXT NOT NULL, -- Guild ID
    permission TEXT NOT NULL, -- Key for the permission
    roles TEXT DEFAULT NULL, -- JSON array as string
    users TEXT DEFAULT NULL, -- JSON array as string
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, permission)
);