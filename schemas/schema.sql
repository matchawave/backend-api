-- sqlite
PRAGMA foreign_keys = ON;

DROP TABLE IF EXISTS guilds;
CREATE TABLE guilds (
    id TEXT PRIMARY KEY,
    enabled INTEGER DEFAULT 0 CHECK (enabled IN (0, 1)), -- 0: FALSE 1: TRUE
    colour TEXT DEFAULT NULL
);

DROP TABLE IF EXISTS guild_settings;
CREATE TABLE guild_settings (
    id TEXT PRIMARY KEY,
    prefix TEXT DEFAULT '!' NOT NULL,
    language TEXT DEFAULT 'en' NOT NULL,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id TEXT PRIMARY KEY
);

DROP TABLE IF EXISTS api_keys;

DROP TABLE IF EXISTS shards;
CREATE TABLE shards (
    shard_id INTEGER PRIMARY KEY,
    status TEXT NOT NULL,
    latency_ms INTEGER NOT NULL DEFAULT -1,
    servers TEXT NOT NULL, -- JSON array as string
    members INTEGER NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- These are tables for all the log configurations
DROP TABLE IF EXISTS message_logs;
CREATE TABLE message_logs(
    id TEXT PRIMARY KEY, -- Server ID
    edited TEXT DEFAULT NULL, -- Channel ID for edited messages event
    deleted TEXT DEFAULT NULL, -- Channel ID for deleted messages event
    commands TEXT DEFAULT NULL, -- Channel ID for command event
    bulk_deleted TEXT DEFAULT NULL, -- Channel ID for bulk deleted messages event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS voice_logs;
CREATE TABLE voice_logs(
    id TEXT PRIMARY KEY, -- Server ID
    joined TEXT DEFAULT NULL, -- Channel ID for user joined voice channel event
    left TEXT DEFAULT NULL, -- Channel ID for user left voice channel event
    switched TEXT DEFAULT NULL, -- Channel ID for user switched voice channel event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS moderation_logs;
CREATE TABLE moderation_logs(
    id TEXT PRIMARY KEY, -- Server ID
    bans TEXT DEFAULT NULL, -- Channel ID for bans event
    unbans TEXT DEFAULT NULL, -- Channel ID for unbans event

    kicks TEXT DEFAULT NULL, -- Channel ID for kicks event

    warns TEXT DEFAULT NULL, -- Channel ID for warns event
    unwarns TEXT DEFAULT NULL, -- Channel ID for unwarns event
    
    mutes TEXT DEFAULT NULL, -- Channel ID for mutes event
    unmutes TEXT DEFAULT NULL, -- Channel ID for unmutes event
    
    timeout TEXT DEFAULT NULL, -- Channel ID for timeouts event
    untimeout TEXT DEFAULT NULL, -- Channel ID for untimeouts event

    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS member_logs;
CREATE TABLE member_logs(
    id TEXT PRIMARY KEY, -- Server ID
    joined TEXT DEFAULT NULL, -- Channel ID for user joined log
    left TEXT DEFAULT NULL, -- Channel ID for user left log
    updated TEXT DEFAULT NULL, -- Channel ID for user updated log
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS channel_logs;
CREATE TABLE channel_logs(
    id TEXT PRIMARY KEY, -- Server ID
    created TEXT DEFAULT NULL, -- Channel ID for channel created event
    deleted TEXT DEFAULT NULL, -- Channel ID for channel deleted event
    updated TEXT DEFAULT NULL, -- Channel ID for channel updated event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS role_logs;
CREATE TABLE role_logs(
    id TEXT PRIMARY KEY, -- Server ID
    created TEXT DEFAULT NULL, -- Channel ID for role created event
    deleted TEXT DEFAULT NULL, -- Channel ID for role deleted event
    updated TEXT DEFAULT NULL, -- Channel ID for role updated event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS emoji_logs;
CREATE TABLE emoji_logs(
    id TEXT PRIMARY KEY, -- Server ID
    created TEXT DEFAULT NULL, -- Channel ID for emoji created event
    deleted TEXT DEFAULT NULL, -- Channel ID for emoji deleted event
    updated TEXT DEFAULT NULL, -- Channel ID for emoji updated event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS guild_logs;
CREATE TABLE guild_logs(
    id TEXT PRIMARY KEY, -- Server ID
    invites TEXT DEFAULT NULL, -- Channel ID for invite created/deleted event
    updated TEXT DEFAULT NULL, -- Channel ID for guild updated event
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
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

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,

    UNIQUE(user_id, guild_id)
);

DROP TABLE IF EXISTS voice_masters;
CREATE TABLE voice_masters(
    id TEXT PRIMARY KEY, -- Server ID
    masters TEXT NOT NULL, -- Channel ID for voice masters
    configs TEXT NOT NULL, -- Channel ID for voice configs

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id) REFERENCES guilds(id) ON DELETE CASCADE
);

CREATE TABLE command_aliases(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    command TEXT NOT NULL,
    alias TEXT NOT NULL,
    args TEXT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    UNIQUE(guild_id, command, alias)
);