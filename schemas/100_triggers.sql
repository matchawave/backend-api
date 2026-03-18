-- Create triggers for user not exists
SELECT group_concat(
    'DROP TRIGGER IF EXISTS user_not_exists_' || name || '; ' ||
    'CREATE TRIGGER user_not_exists_' || name || 
    ' BEFORE INSERT ON ' || name || 
    ' FOR EACH ROW ' ||
    ' BEGIN' ||
    ' INSERT OR IGNORE INTO users(id) VALUES (NEW.user_id); ' ||
    ' END;', 
    ' '
) FROM sqlite_master
WHERE type = 'table' 
AND name IN (
    'afk_statuses', 'afk_configs', 
    'user_levels', 'birthdays', 'permission_users', 'embeds', 'reaction_triggers', 
    'guild_ignore_users', 'voice_configs', 'giveaways', 'giveaway_entries'
);

-- Create triggers for when guild is updated
SELECT group_concat(
    'DROP TRIGGER IF EXISTS guild_updated_' || name || '; ' ||
    'CREATE TRIGGER guild_updated_' || name || 
    ' AFTER UPDATE ON ' || name || 
    ' FOR EACH ROW ' ||
    ' BEGIN' ||
    ' UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id; ' ||
    ' END;', 
    ' DROP TRIGGER IF EXISTS guild_inserted_' || name || '; ' ||
    'CREATE TRIGGER guild_inserted_' || name || 
    ' AFTER INSERT ON ' || name || 
    ' FOR EACH ROW ' ||
    ' BEGIN' ||
    ' UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.guild_id; ' ||
    ' END;' ||
    'DROP TRIGGER IF EXISTS guild_deleted_' || name || '; ' ||
    'CREATE TRIGGER guild_deleted_' || name || 
    ' AFTER DELETE ON ' || name || 
    ' FOR EACH ROW ' ||
    ' BEGIN' ||
    ' UPDATE guilds SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.guild_id; ' ||
    ' END;'
) FROM sqlite_master
WHERE type = 'table' 
AND name IN (
    'guild_settings', 'command_aliases', 
    'level_configs', 'level_roles', 'level_xp_multipliers', 
    'birthday_configs', 'embeds',
    'permission_roles', 'permission_users',
    'reaction_triggers', 'reaction_channels',
    'guild_log_configs', 'guild_ignore_channels', 'guild_ignore_users',
    'voice_masters', 'voice_configs'
);