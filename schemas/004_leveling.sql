DROP TABLE IF EXISTS user_levels;
CREATE TABLE user_levels(
    user_id TEXT NOT NULL, -- User ID
    guild_id TEXT NOT NULL, -- Guild ID
    level INTEGER NOT NULL DEFAULT 0, -- User's current level
    xp INTEGER NOT NULL DEFAULT 0, -- User's current XP
    last_message_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the user last sent a message (for cooldown)
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, guild_id)
);

DROP TABLE IF EXISTS user_profiles;
CREATE TABLE user_profiles(
    user_id TEXT PRIMARY KEY, -- User ID
    avatar_url TEXT DEFAULT NULL, -- Custom avatar URL for level card
    background_url TEXT DEFAULT NULL, -- Custom background URL for level card
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS level_configs;
CREATE TABLE level_configs(
    guild_id TEXT PRIMARY KEY, -- Guild ID
    minimum_xp_gain INTEGER NOT NULL DEFAULT 15, -- Minimum XP gain per message
    maximum_xp_gain INTEGER NOT NULL DEFAULT 25, -- Maximum XP gain per message
    level_up_message TEXT NOT NULL DEFAULT 'GGs {user}, you have reached level {level.rank}!', -- Message to show when user levels up
    channel_id TEXT DEFAULT NULL, -- Channel ID to send level up messages in, NULL for current channel
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS level_roles;
CREATE TABLE level_roles(
    guild_id TEXT NOT NULL, -- Guild ID
    role_id TEXT NOT NULL, -- Role ID to assign
    level INTEGER NOT NULL, -- Level required for the role
    stackable BOOLEAN NOT NULL DEFAULT 0 CHECK(stackable IN (0, 1)), -- Whether the role is stackable with other level roles
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, role_id)
);

DROP TABLE IF EXISTS level_xp_multipliers;
CREATE TABLE level_xp_multipliers(
    guild_id TEXT NOT NULL, -- Guild ID
    role_id TEXT DEFAULT NULL, -- Role ID for role-specific multiplier, NULL for guild-wide multiplier
    multiplier REAL NOT NULL DEFAULT 1.0 CHECK(multiplier > 0 AND multiplier < 10), -- XP multiplier for the guild
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, role_id)
);

DROP TRIGGER IF EXISTS user_not_exists_user_levels;
CREATE TRIGGER user_not_exists_user_levels
BEFORE INSERT ON user_levels
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_user_levels;
CREATE TRIGGER guild_inserted_user_levels
AFTER INSERT ON user_levels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_user_levels;
CREATE TRIGGER guild_updated_user_levels
AFTER UPDATE ON user_levels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_user_levels;
CREATE TRIGGER guild_deleted_user_levels
AFTER DELETE ON user_levels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS user_not_exists_user_profiles;
CREATE TRIGGER user_not_exists_user_profiles
BEFORE INSERT ON user_profiles
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_level_configs;
CREATE TRIGGER guild_inserted_level_configs
AFTER INSERT ON level_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_level_configs;
CREATE TRIGGER guild_updated_level_configs
AFTER UPDATE ON level_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_level_configs;
CREATE TRIGGER guild_deleted_level_configs
AFTER DELETE ON level_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_level_roles;
CREATE TRIGGER guild_inserted_level_roles
AFTER INSERT ON level_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_level_roles;
CREATE TRIGGER guild_updated_level_roles
AFTER UPDATE ON level_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_level_roles;
CREATE TRIGGER guild_deleted_level_roles
AFTER DELETE ON level_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_level_xp_multipliers;
CREATE TRIGGER guild_inserted_level_xp_multipliers
AFTER INSERT ON level_xp_multipliers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_level_xp_multipliers;
CREATE TRIGGER guild_updated_level_xp_multipliers
AFTER UPDATE ON level_xp_multipliers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_level_xp_multipliers;
CREATE TRIGGER guild_deleted_level_xp_multipliers
AFTER DELETE ON level_xp_multipliers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

