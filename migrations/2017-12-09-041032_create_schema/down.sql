
DROP TRIGGER IF EXISTS update_git_tags_last_modified ON git_tags;
DROP TRIGGER IF EXISTS update_language_stats_last_modified ON language_stats;
DROP TRIGGER IF EXISTS update_languages_last_modified ON languages;
DROP TRIGGER IF EXISTS update_completed_parses_last_modified ON git_repos;
DROP TRIGGER IF EXISTS update_parses_last_modified ON parses;
DROP TRIGGER IF EXISTS update_git_commits_last_modified ON git_repos;
DROP TRIGGER IF EXISTS update_git_repos_last_modified ON git_repos;

DROP TABLE IF EXISTS git_commits;
DROP TABLE IF EXISTS git_tags;
DROP TABLE IF EXISTS language_stats;
DROP TABLE IF EXISTS languages;
DROP TABLE IF EXISTS completed_parses;
DROP TABLE IF EXISTS parses;
DROP TABLE IF EXISTS git_repos;

DROP FUNCTION IF EXISTS update_lastmodified_timestamp_column();
