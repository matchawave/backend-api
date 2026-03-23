DROP TABLE IF EXISTS guild_log_configs;
CREATE TABLE guild_log_configs (
    guild_id TEXT NOT NULL,
    log_type TEXT NOT NULL CHECK(log_type IN ('message', 'voice', 'moderation', 'member', 'channel', 'role', 'emoji', 'guild')), -- Type of log (e.g., message, voice, moderation)
    data TEXT NOT NULL, -- JSON data as string
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, log_type)
);

DROP TABLE IF EXISTS guild_ignore_channels;
CREATE TABLE guild_ignore_channels (
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, channel_id)
);

DROP TABLE IF EXISTS guild_ignore_users;
CREATE TABLE guild_ignore_users (
    guild_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, user_id)
);

DROP TRIGGER IF EXISTS guild_inserted_guild_log_configs;
CREATE TRIGGER guild_inserted_guild_log_configs
AFTER INSERT ON guild_log_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_log_configs;
CREATE TRIGGER guild_updated_guild_log_configs
AFTER UPDATE ON guild_log_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_log_configs;
CREATE TRIGGER guild_deleted_guild_log_configs
AFTER DELETE ON guild_log_configs
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_guild_ignore_channels;
CREATE TRIGGER guild_inserted_guild_ignore_channels
AFTER INSERT ON guild_ignore_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_ignore_channels;
CREATE TRIGGER guild_updated_guild_ignore_channels
AFTER UPDATE ON guild_ignore_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_ignore_channels;
CREATE TRIGGER guild_deleted_guild_ignore_channels
AFTER DELETE ON guild_ignore_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_guild_ignore_users;
CREATE TRIGGER guild_inserted_guild_ignore_users
AFTER INSERT ON guild_ignore_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_ignore_users;
CREATE TRIGGER guild_updated_guild_ignore_users
AFTER UPDATE ON guild_ignore_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_ignore_users;
CREATE TRIGGER guild_deleted_guild_ignore_users
AFTER DELETE ON guild_ignore_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS user_not_exists_guild_ignore_users;
CREATE TRIGGER user_not_exists_guild_ignore_users
BEFORE INSERT ON guild_ignore_users
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;