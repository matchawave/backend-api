DROP TABLE IF EXISTS birthdays;
CREATE TABLE birthdays (
    user_id TEXT PRIMARY KEY, -- User ID
    day INTEGER NOT NULL CHECK(day BETWEEN 1 AND 31), -- Day of the month (1-31)
    month INTEGER NOT NULL CHECK(month BETWEEN 1 AND 12), -- Month of the year (1-12)
    year INTEGER DEFAULT NULL, -- Year of birth / Can be null if the user doesn't want to share their age
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS birthday_configs;
CREATE TABLE birthday_configs (
    guild_id TEXT PRIMARY KEY, -- Guild ID
    channel_id TEXT DEFAULT NULL, -- Channel ID to send birthday messages in, NULL for current channel
    message TEXT NOT NULL DEFAULT 'Happy Birthday {user}! 🎉', -- Message to show when it's a user's birthday
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);


