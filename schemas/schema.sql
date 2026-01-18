-- sqlite
PRAGMA foreign_keys = ON;

DROP TABLE IF EXISTS shards;
CREATE TABLE shards (
    id INTEGER PRIMARY KEY, -- Shard ID
    status TEXT NOT NULL, -- e.g., online, offline, connecting
    latency INTEGER DEFAULT NULL, -- Latency in milliseconds
    members INTEGER NOT NULL, -- Count of members across all guilds in this shard
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS guilds;
CREATE TABLE guilds (
    id TEXT PRIMARY KEY, -- Guild ID
    enabled INTEGER DEFAULT 0 CHECK (enabled IN (0, 1)), -- 0: FALSE 1: TRUE 
    shard_id INTEGER NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the bot was added to the guild
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the guild activity was last recorded
    FOREIGN KEY (shard_id) REFERENCES shards(id) ON DELETE CASCADE
);

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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, log_type)
);

DROP TABLE IF EXISTS voice_configs;
CREATE TABLE voice_configs(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    guild_id TEXT DEFAULT NULL, -- NULL for non-guild config

    name TEXT DEFAULT NULL,
    bitrate INTEGER DEFAULT NULL,
    user_limit INTEGER DEFAULT NULL,
    locked TEXT DEFAULT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,

    UNIQUE(user_id, guild_id)
);

DROP TABLE IF EXISTS voice_masters;
CREATE TABLE voice_masters(
    guild_id TEXT PRIMARY KEY, -- Server ID
    masters TEXT NOT NULL, -- Channel ID for voice masters
    configs TEXT NOT NULL, -- Channel ID for voice configs

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS command_aliases;
CREATE TABLE command_aliases(
    guild_id TEXT NOT NULL,
    command TEXT NOT NULL,
    alias TEXT NOT NULL,
    args TEXT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
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
