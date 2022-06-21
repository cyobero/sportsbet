table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    events (id, timestamp) {
        id -> Int4,
        description -> Varchar,
        odds -> Int4,
        result_id -> Nullable<Int4>,
        timestamp -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    game_results (id) {
        id -> Int4,
        home -> Int4,
        away -> Int4,
        game_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    games (id) {
        id -> Int4,
        home -> Varchar,
        away -> Varchar,
        start -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        role -> Role,
    }
}

joinable!(events -> game_results (result_id));
joinable!(game_results -> games (game_id));

allow_tables_to_appear_in_same_query!(
    events,
    game_results,
    games,
    users,
);
