DROP TABLE IF EXISTS embeds;
CREATE TABLE embeds (
    name TEXT NOT NULL, -- Name of the embed config, e.g., "welcome", "goodbye", "announcement"
    guild_id TEXT NOT NULL, -- Guild ID for server-specific embed
    user_id TEXT NOT NULL, -- User ID of the person who created the embed config
    content TEXT DEFAULT NULL,
    embeds TEXT DEFAULT "[]", -- JSON array as string
    components TEXT DEFAULT "[]", -- JSON data as string
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, name)
);

DROP TRIGGER IF EXISTS user_not_exists_embeds;
CREATE TRIGGER user_not_exists_embeds
BEFORE INSERT ON embeds
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_embeds;
CREATE TRIGGER guild_inserted_embeds
AFTER INSERT ON embeds
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_embeds;
CREATE TRIGGER guild_updated_embeds
AFTER UPDATE ON embeds
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_embeds;
CREATE TRIGGER guild_deleted_embeds
AFTER DELETE ON embeds
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;
