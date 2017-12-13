table! {
    completed_parses (completed_parse_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        completed_parse_id -> Int4,
        parse_id -> Nullable<Int4>,
    }
}

table! {
    git_repos (git_repo_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        git_repo_id -> Int4,
        origin_remote -> Nullable<Text>,
    }
}

table! {
    git_tags (git_tag_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        git_tag_id -> Int8,
        git_repo_id -> Nullable<Int4>,
        git_tag -> Text,
    }
}

table! {
    languages (language_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        language_id -> Int8,
        parse_id -> Nullable<Int4>,
        name -> Text,
        blanks -> Int8,
        code -> Int8,
        comments -> Int8,
        lines -> Int8,
        nested -> Bool,
        files -> Nullable<Array<Text>>,
    }
}

table! {
    language_stats (language_stat_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        language_stat_id -> Int8,
        language_id -> Nullable<Int8>,
        parse_id -> Nullable<Int4>,
        name -> Text,
        blanks -> Int8,
        code -> Int8,
        comments -> Int8,
        lines -> Int8,
    }
}

table! {
    parses (parse_id) {
        created_on -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        parse_id -> Int4,
        git_repo_id -> Nullable<Int4>,
        time -> Timestamptz,
        git_tag -> Nullable<Text>,
    }
}

joinable!(completed_parses -> parses (parse_id));
joinable!(git_tags -> git_repos (git_repo_id));
joinable!(language_stats -> languages (language_id));
joinable!(language_stats -> parses (parse_id));
joinable!(languages -> parses (parse_id));
joinable!(parses -> git_repos (git_repo_id));

allow_tables_to_appear_in_same_query!(
    completed_parses,
    git_repos,
    git_tags,
    languages,
    language_stats,
    parses,
);
