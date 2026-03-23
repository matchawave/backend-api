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
    user_id TEXT NOT NULL, -- User ID to assign
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, permission, user_id)
);

DROP TRIGGER IF EXISTS user_not_exists_permission_users;
CREATE TRIGGER user_not_exists_permission_users
BEFORE INSERT ON permission_users
FOR EACH ROW
BEGIN
    INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id);
END;

DROP TRIGGER IF EXISTS guild_inserted_permission_users;
CREATE TRIGGER guild_inserted_permission_users
AFTER INSERT ON permission_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_permission_users;
CREATE TRIGGER guild_updated_permission_users
AFTER UPDATE ON permission_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_permission_users;
CREATE TRIGGER guild_deleted_permission_users
AFTER DELETE ON permission_users
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_permission_roles;
CREATE TRIGGER guild_inserted_permission_roles
AFTER INSERT ON permission_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_permission_roles;
CREATE TRIGGER guild_updated_permission_roles
AFTER UPDATE ON permission_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_permission_roles;
CREATE TRIGGER guild_deleted_permission_roles
AFTER DELETE ON permission_roles
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;