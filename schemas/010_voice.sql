DROP TABLE IF EXISTS voice_masters;
CREATE TABLE voice_masters(
    guild_id TEXT KEY, -- Server ID
    master_id TEXT NOT NULL, -- Create channel
    category_id TEXT DEFAULT NULL, -- Category for created channels

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, master_id)
);

DROP TABLE IF EXISTS voice_configs;
CREATE TABLE voice_configs(
    user_id TEXT DEFAULT NULL, -- NULL for guild configs
    guild_id TEXT DEFAULT NULL, -- NULL for non-guild config

    name TEXT DEFAULT NULL,
    bitrate INTEGER DEFAULT NULL,
    user_limit INTEGER DEFAULT NULL,
    locked INTEGER DEFAULT 0 CHECK(locked IN (0, 1)),

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, guild_id)
);