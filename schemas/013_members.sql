DROP TABLE IF EXISTS guild_members;
CREATE TABLE guild_members (
    user_id TEXT NOT NULL,
    guild_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (guild_id) REFERENCES guilds(id),
    PRIMARY KEY (user_id, guild_id)
);

DROP TRIGGER IF EXISTS user_not_exists_members;
CREATE TRIGGER user_not_exists_members
BEFORE INSERT ON guild_members
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;