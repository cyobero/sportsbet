table! {
    events (id, timestamp) {
        id -> Int4,
        description -> Varchar,
        odds -> Int4,
        result_id -> Nullable<Int4>,
        timestamp -> Timestamp,
    }
}

table! {
    game_results (id) {
        id -> Int4,
        home -> Int4,
        away -> Int4,
        game_id -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        home -> Varchar,
        away -> Varchar,
        start -> Timestamp,
    }
}

joinable!(events -> game_results (result_id));
joinable!(game_results -> games (game_id));

allow_tables_to_appear_in_same_query!(
    events,
    game_results,
    games,
);
