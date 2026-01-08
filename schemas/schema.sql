PRAGMA foreign_keys = ON;

DROP TABLE IF EXISTS guilds;
CREATE TABLE guilds (
    id TEXT PRIMARY KEY,
    enabled INTEGER DEFAULT 0 CHECK (enabled IN (0, 1))
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
CREATE TABLE api_keys (
    key TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    guild_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS shards;
CREATE TABLE shards (
    shard_id INTEGER PRIMARY KEY,
    status TEXT NOT NULL,
    latency_ms INTEGER NOT NULL DEFAULT -1,
    servers TEXT NOT NULL, -- JSON array as string
    members INTEGER NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
