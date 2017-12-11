table! {
    git_repos (git_repo_id) {
        git_repo_id -> Int4,
        origin_remote -> Nullable<Text>,
    }
}

table! {
    git_tags (git_tag_id) {
        git_tag_id -> Int8,
        git_repo_id -> Nullable<Int4>,
        git_tag -> Text,
    }
}

table! {
    languages (language_id) {
        language_id -> Int8,
        parse_id -> Nullable<Int4>,
        name -> Text,
        blanks -> Int8,
        code -> Int8,
        comments -> Int8,
        lines -> Int8,
        nested -> Bool,
    }
}

table! {
    language_stats (language_stat_id) {
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
        parse_id -> Int4,
        git_repo_id -> Nullable<Int4>,
        time -> Timestamptz,
        git_tag -> Nullable<Text>,
    }
}

joinable!(git_tags -> git_repos (git_repo_id));
joinable!(language_stats -> languages (language_id));
joinable!(language_stats -> parses (parse_id));
joinable!(languages -> parses (parse_id));
joinable!(parses -> git_repos (git_repo_id));

allow_tables_to_appear_in_same_query!(
    git_repos,
    git_tags,
    languages,
    language_stats,
    parses,
);
