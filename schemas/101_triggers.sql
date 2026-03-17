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
