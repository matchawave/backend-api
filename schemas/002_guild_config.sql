DROP TABLE IF EXISTS guild_prefixes;
CREATE TABLE guild_prefixes (
    guild_id TEXT PRIMARY KEY NOT NULL,
    prefix TEXT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS guild_timezones;
CREATE TABLE guild_timezones (
    guild_id TEXT PRIMARY KEY NOT NULL,
    timezone TEXT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS guild_languages;
CREATE TABLE guild_languages (
    guild_id TEXT PRIMARY KEY NOT NULL,
    language TEXT NOT NULL CHECK(language IN ('english', 'français')),
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS guild_colours;
CREATE TABLE guild_colours (
    guild_id TEXT PRIMARY KEY NOT NULL,
    colour TEXT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

DROP TRIGGER IF EXISTS guild_inserted_guild_prefixes;
CREATE TRIGGER guild_inserted_guild_prefixes
AFTER INSERT ON guild_prefixes
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_prefixes;
CREATE TRIGGER guild_updated_guild_prefixes
AFTER UPDATE ON guild_prefixes
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_prefixes;
CREATE TRIGGER guild_deleted_guild_prefixes
AFTER DELETE ON guild_prefixes
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_guild_languages;
CREATE TRIGGER guild_inserted_guild_languages
AFTER INSERT ON guild_languages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_languages;
CREATE TRIGGER guild_updated_guild_languages
AFTER UPDATE ON guild_languages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_languages;
CREATE TRIGGER guild_deleted_guild_languages
AFTER DELETE ON guild_languages
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_guild_timezones;
CREATE TRIGGER guild_inserted_guild_timezones
AFTER INSERT ON guild_timezones
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_timezones;
CREATE TRIGGER guild_updated_guild_timezones
AFTER UPDATE ON guild_timezones
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_timezones;
CREATE TRIGGER guild_deleted_guild_timezones
AFTER DELETE ON guild_timezones
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

DROP TRIGGER IF EXISTS guild_inserted_guild_colours;
CREATE TRIGGER guild_inserted_guild_colours
AFTER INSERT ON guild_colours
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_updated_guild_colours;
CREATE TRIGGER guild_updated_guild_colours
AFTER UPDATE ON guild_colours
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id;
END;

DROP TRIGGER IF EXISTS guild_deleted_guild_colours;
CREATE TRIGGER guild_deleted_guild_colours
AFTER DELETE ON guild_colours
FOR EACH ROW
BEGIN
    UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id;
END;

