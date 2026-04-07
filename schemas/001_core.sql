DROP TABLE IF EXISTS guild_settings;

DROP TABLE IF EXISTS shards;
CREATE TABLE shards (
    id INTEGER PRIMARY KEY, -- Shard ID
    started_at TIMESTAMP DEFAULT NULL -- When the shard was started
);

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id TEXT PRIMARY KEY, -- User ID
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- When the user was first seen by the system
);

DROP TABLE IF EXISTS guilds;
CREATE TABLE guilds (
    id TEXT PRIMARY KEY, -- Guild ID
    enabled BOOLEAN NOT NULL DEFAULT 1 CHECK(enabled IN (0, 1)), -- Whether the bot is active in this guild
    shard_id INTEGER NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the bot was added to the guild
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the guild activity was last recorded
    FOREIGN KEY (shard_id) REFERENCES shards(id) ON DELETE CASCADE
);