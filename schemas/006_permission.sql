DROP TABLE IF EXISTS permission_roles;
CREATE TABLE permission_roles (
    guild_id TEXT NOT NULL, -- Guild ID
    permission TEXT NOT NULL, -- Key for the permission
    role_id TEXT DEFAULT NULL, -- Role ID to assign 
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, permission)
);

DROP TABLE IF EXISTS permission_users;
CREATE TABLE permission_users (
    guild_id TEXT NOT NULL, -- Guild ID
    permission TEXT NOT NULL, -- Key for the permission
    user_id TEXT DEFAULT NULL, -- User ID to assign 
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, permission)
);
