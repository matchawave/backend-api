DROP TABLE IF EXISTS reminders;
CREATE TABLE reminders(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL, -- User ID of the person to remind
    guild_id TEXT NOT NULL, -- Guild ID for server-specific reminders, NULL for global reminders
    channel_id TEXT DEFAULT NULL, -- Channel ID to send the reminder in, NULL for DM
    message TEXT NOT NULL, -- Reminder message
    remind_at TIMESTAMP NOT NULL, -- When to send the reminder
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS timed_messages;
CREATE TABLE timed_messages(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL, -- Guild ID to send the message in
    channel_id TEXT NOT NULL, -- Channel ID to send the message in
    message TEXT NOT NULL, -- Message to send
    interval TEXT NOT NULL, -- Interval for sending the message (e.g., 'daily', 'weekly', 'monthly')
    send_at TIMESTAMP NOT NULL, -- When to send the message
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);