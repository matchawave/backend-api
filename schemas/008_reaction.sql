DROP TABLE IF EXISTS reaction_triggers;
CREATE TABLE reaction_triggers (
    guild_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    trigger TEXT NOT NULL,
    author_id TEXT NOT NULL, -- User ID of the person who created the trigger
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, trigger, emoji)
);

DROP TABLE IF EXISTS reaction_channels;
CREATE TABLE reaction_channels (
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, channel_id, emoji)
);

DROP TRIGGER IF EXISTS user_not_exists_reaction_triggers;
CREATE TRIGGER user_not_exists_reaction_triggers
BEFORE INSERT ON reaction_triggers
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_reaction_triggers;
CREATE TRIGGER guild_inserted_reaction_triggers
AFTER INSERT ON reaction_triggers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_reaction_triggers;
CREATE TRIGGER guild_updated_reaction_triggers
AFTER UPDATE ON reaction_triggers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_reaction_triggers;
CREATE TRIGGER guild_deleted_reaction_triggers
AFTER DELETE ON reaction_triggers
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_reaction_channels;
CREATE TRIGGER guild_inserted_reaction_channels
AFTER INSERT ON reaction_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_reaction_channels;
CREATE TRIGGER guild_updated_reaction_channels
AFTER UPDATE ON reaction_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_reaction_channels;
CREATE TRIGGER guild_deleted_reaction_channels
AFTER DELETE ON reaction_channels
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;