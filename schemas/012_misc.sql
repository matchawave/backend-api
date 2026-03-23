DROP TABLE IF EXISTS reminders;
CREATE TABLE reminders(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL, -- User ID of the person to remind
    guild_id TEXT NOT NULL, -- Guild ID for server-specific reminders, NULL for global reminders
    channel_id TEXT DEFAULT NULL, -- Channel ID to send the reminder in, NULL for DM
    message TEXT NOT NULL, -- Reminder message
    remind_at TIMESTAMP NOT NULL, -- When to send the reminder
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS timed_messages;
CREATE TABLE timed_messages(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL, -- Guild ID to send the message in
    channel_id TEXT NOT NULL, -- Channel ID to send the message in
    message TEXT NOT NULL, -- Message to send
    interval INTEGER NOT NULL, -- Interval for sending the message, stored in seconds
    author_id TEXT NOT NULL, -- User ID of the person who created the timed message
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TRIGGER IF EXISTS user_not_exists_reminders;
CREATE TRIGGER user_not_exists_reminders
BEFORE INSERT ON reminders
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS user_not_exists_timed_messages;
CREATE TRIGGER user_not_exists_timed_messages   
BEFORE INSERT ON timed_messages
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_reminders;
CREATE TRIGGER guild_inserted_reminders
AFTER INSERT ON reminders
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_reminders;
CREATE TRIGGER guild_updated_reminders
AFTER UPDATE ON reminders
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_reminders;
CREATE TRIGGER guild_deleted_reminders
AFTER DELETE ON reminders
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_timed_messages;
CREATE TRIGGER guild_inserted_timed_messages
AFTER INSERT ON timed_messages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_timed_messages;
CREATE TRIGGER guild_updated_timed_messages
AFTER UPDATE ON timed_messages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_timed_messages;
CREATE TRIGGER guild_deleted_timed_messages
AFTER DELETE ON timed_messages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;