-- Leveling Tables --
DROP TABLE IF EXISTS user_levels;
CREATE TABLE user_levels(
    user_id TEXT NOT NULL, -- User ID
    guild_id TEXT NOT NULL, -- Guild ID
    level INTEGER NOT NULL DEFAULT 0, -- User's current level
    xp INTEGER NOT NULL DEFAULT 0, -- User's current XP
    last_message_at TIMESTAMP DEFAULT (datetime('now', 'utc')), -- When the user last sent a message (for cooldown)
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, guild_id)
);

DROP TABLE IF EXISTS user_level_profiles;
CREATE TABLE user_level_profiles(
    user_id TEXT PRIMARY KEY, -- User ID
    avatar_url TEXT DEFAULT NULL, -- Custom avatar URL for level card
    background_url TEXT DEFAULT NULL, -- Custom background URL for level card
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS level_configs;
CREATE TABLE level_configs(
    guild_id TEXT PRIMARY KEY, -- Guild ID
    minimum_xp_gain INTEGER NOT NULL DEFAULT 15, -- Minimum XP gain per message
    maximum_xp_gain INTEGER NOT NULL DEFAULT 25, -- Maximum XP gain per message
    level_up_message TEXT NOT NULL DEFAULT 'GGs {user}, you have reached level {level.rank}!', -- Message to show when user levels up
    channel_id TEXT DEFAULT NULL, -- Channel ID to send level up messages in, NULL for current channel
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS level_roles;
CREATE TABLE level_roles(
    guild_id TEXT NOT NULL, -- Guild ID
    level INTEGER NOT NULL, -- Level required for the role
    role_id TEXT NOT NULL, -- Role ID to assign
    stackable BOOLEAN NOT NULL DEFAULT 1, -- Whether the role is stackable with other level roles
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, level)
);

DROP TABLE IF EXISTS level_xp_multipliers;
CREATE TABLE level_xp_multipliers(
    guild_id TEXT NOT NULL, -- Guild ID
    multiplier REAL NOT NULL DEFAULT 1.0, -- XP multiplier for the guild
    channel_id TEXT DEFAULT NULL, -- Channel ID for channel-specific multiplier, NULL for guild-wide multiplier
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, channel_id)
);