DROP TABLE IF EXISTS command_aliases;
CREATE TABLE command_aliases(
    guild_id TEXT NOT NULL,
    command TEXT NOT NULL,
    alias TEXT NOT NULL,
    args TEXT DEFAULT NULL,
    author_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, command, alias)
);

DROP TRIGGER IF EXISTS guild_inserted_command_aliases;
CREATE TRIGGER guild_inserted_command_aliases
AFTER INSERT ON command_aliases
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_command_aliases;
CREATE TRIGGER guild_updated_command_aliases
AFTER UPDATE ON command_aliases
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_command_aliases;
CREATE TRIGGER guild_deleted_command_aliases
AFTER DELETE ON command_aliases
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS user_not_exists_command_aliases;
CREATE TRIGGER user_not_exists_command_aliases
BEFORE INSERT ON command_aliases
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.author_id);
END;
