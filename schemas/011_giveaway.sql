DROP TABLE IF EXISTS giveaways;
CREATE TABLE giveaways(
    giveaway_id TEXT PRIMARY KEY, -- Giveaway ID (unique identifier)
    guild_id TEXT NOT NULL, -- Guild ID where the giveaway is hosted
    channel_id TEXT NOT NULL, -- Channel ID where the giveaway is hosted
    message_id TEXT NOT NULL, -- Message ID of the giveaway announcement
    prize TEXT NOT NULL, -- Prize for the giveaway
    end_time TIMESTAMP NOT NULL, -- When the giveaway ends
    winners_count INTEGER NOT NULL, -- Number of winners to select
    host_id TEXT NOT NULL, -- User ID of the giveaway host
    ended BOOLEAN DEFAULT 0, -- Whether the giveaway has ended
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    updated_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (host_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS giveaway_entries;
CREATE TABLE giveaway_entries(
    giveaway_id TEXT NOT NULL, -- Giveaway ID (foreign key to giveaways table)
    user_id TEXT NOT NULL, -- User ID of the participant
    created_at TIMESTAMP DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (giveaway_id) REFERENCES giveaways(giveaway_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (giveaway_id, user_id)
);