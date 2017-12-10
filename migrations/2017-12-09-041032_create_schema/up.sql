
CREATE TABLE IF NOT EXISTS parses (
       parse_id serial primary key,
       time timestamp with time zone NOT NULL,
       git_tag text
);

CREATE TABLE IF NOT EXISTS languages (
       language_id bigserial primary key,
       parse_id integer references parses(parse_id),
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
       language_stat_id bigserial primary key,
       language_id bigint references languages(language_id),
       name text NOT NULL,
       blanks bigint NOT NULL,
       code bigint NOT NULL,
       comments bigint NOT NULL,
       lines bigint NOT NULL
);
