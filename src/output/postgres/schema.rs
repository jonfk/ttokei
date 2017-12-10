
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
        time -> Timestamptz,
        git_tag -> Nullable<Text>,
    }
}

joinable!(language_stats -> languages (language_id));
joinable!(language_stats -> parses (parse_id));
joinable!(languages -> parses (parse_id));

allow_tables_to_appear_in_same_query!(
    languages,
    language_stats,
    parses,
);
