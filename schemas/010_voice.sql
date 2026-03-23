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

DROP TRIGGER IF EXISTS user_not_exists_voice_configs;
CREATE TRIGGER user_not_exists_voice_configs
BEFORE INSERT ON voice_configs
FOR EACH ROW
WHEN NEW.user_id IS NOT NULL
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_voice_configs;
CREATE TRIGGER guild_inserted_voice_configs
AFTER INSERT ON voice_configs
FOR EACH ROW
WHEN NEW.guild_id IS NOT NULL
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_voice_configs;
CREATE TRIGGER guild_updated_voice_configs
AFTER UPDATE ON voice_configs
FOR EACH ROW
WHEN NEW.guild_id IS NOT NULL
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_voice_configs;
CREATE TRIGGER guild_deleted_voice_configs
AFTER DELETE ON voice_configs
FOR EACH ROW
WHEN OLD.guild_id IS NOT NULL
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_voice_masters;
CREATE TRIGGER guild_inserted_voice_masters
AFTER INSERT ON voice_masters
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_voice_masters;
CREATE TRIGGER guild_updated_voice_masters
AFTER UPDATE ON voice_masters
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_voice_masters;
CREATE TRIGGER guild_deleted_voice_masters
AFTER DELETE ON voice_masters
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;