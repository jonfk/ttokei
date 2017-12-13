
CREATE TABLE IF NOT EXISTS git_repos (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       git_repo_id serial primary key,
       origin_remote text UNIQUE
);

CREATE TABLE IF NOT EXISTS git_tags (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       git_tag_id bigserial primary key,
       git_repo_id integer references git_repos(git_repo_id),
       git_tag text NOT NULL,
       UNIQUE(git_repo_id, git_tag)
);

CREATE TABLE IF NOT EXISTS parses (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       parse_id serial primary key,
       git_repo_id integer references git_repos(git_repo_id),
       time timestamp with time zone NOT NULL,
       git_tag text
);

CREATE TABLE IF NOT EXISTS completed_parses (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       completed_parse_id serial primary key,
       parse_id integer references parses(parse_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS languages (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       language_id bigserial primary key,
       parse_id integer references parses(parse_id) ON DELETE CASCADE,
       name text NOT NULL,
       blanks bigint NOT NULL,
       code bigint NOT NULL,
       comments bigint NOT NULL,
       lines bigint NOT NULL,
       nested boolean NOT NULL
       --files text[],
       --line_comment text[],
       --multi_line text[][],
       --nested_comments text[][],
       --quotes text[][]
);

CREATE TABLE IF NOT EXISTS language_stats (
       created_on timestamp default now(),
       last_modified timestamp default now(),
       language_stat_id bigserial primary key,
       language_id bigint references languages(language_id) ON DELETE CASCADE,
       parse_id integer references parses(parse_id) ON DELETE CASCADE,
       name text NOT NULL,
       blanks bigint NOT NULL,
       code bigint NOT NULL,
       comments bigint NOT NULL,
       lines bigint NOT NULL
);

-- triggers
CREATE OR REPLACE FUNCTION update_lastmodified_timestamp_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.last_modified = now();
  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_git_repos_last_modified BEFORE UPDATE
ON git_repos FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();

CREATE TRIGGER update_git_tags_last_modified BEFORE UPDATE
ON git_tags FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();

CREATE TRIGGER update_parses_last_modified BEFORE UPDATE
ON parses FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();

CREATE TRIGGER update_completed_parses_last_modified BEFORE UPDATE
ON completed_parses FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();

CREATE TRIGGER update_languages_last_modified BEFORE UPDATE
ON languages FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();

CREATE TRIGGER update_language_stats_last_modified BEFORE UPDATE
ON language_stats FOR EACH ROW EXECUTE PROCEDURE
update_lastmodified_timestamp_column();
