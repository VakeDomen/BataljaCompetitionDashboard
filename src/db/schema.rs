// @generated automatically by Diesel CLI.

diesel::table! {
    bots (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        team_id -> Varchar,
        #[max_length = 255]
        bot_name -> Varchar,
        #[max_length = 255]
        source_path -> Varchar,
        created -> Datetime,
    }
}

diesel::table! {
    competitions (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        start -> Datetime,
        end -> Datetime,
        #[max_length = 255]
        allowed_submissions -> Varchar,
        #[max_length = 255]
        round -> Varchar,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        created -> Datetime,
    }
}

diesel::table! {
    games_2v2 (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        competition_id -> Varchar,
        #[max_length = 255]
        round -> Varchar,
        #[max_length = 255]
        team1_id -> Varchar,
        #[max_length = 255]
        team2_id -> Varchar,
        #[max_length = 255]
        team1bot1_id -> Varchar,
        #[max_length = 255]
        team1bot2_id -> Varchar,
        #[max_length = 255]
        team2bot1_id -> Varchar,
        #[max_length = 255]
        team2bot2_id -> Varchar,
        team1bot1_survived -> Bool,
        team1bot2_survived -> Bool,
        team2bot1_survived -> Bool,
        team2bot2_survived -> Bool,
        #[max_length = 4096]
        log_file_path -> Varchar,
        public -> Bool,
        additional_data -> Text,
        created -> Datetime,
    }
}

diesel::table! {
    teams (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        #[max_length = 255]
        partner -> Varchar,
        #[max_length = 255]
        competition_id -> Varchar,
        #[max_length = 255]
        bot1 -> Varchar,
        #[max_length = 255]
        bot2 -> Varchar,
        created -> Datetime,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        ldap_dn -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        created -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bots,
    competitions,
    games_2v2,
    teams,
    users,
);
