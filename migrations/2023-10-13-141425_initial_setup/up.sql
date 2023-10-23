CREATE TABLE users (
    id              VARCHAR(255) NOT NULL PRIMARY KEY,
    username        VARCHAR(255) NOT NULL,
    ldap_dn         VARCHAR(255) NOT NULL,
    role            VARCHAR(255) NOT NULL,
    created         DATETIME NOT NULL
);

CREATE TABLE competitions (
    id                      VARCHAR(255) NOT NULL PRIMARY KEY,
    name                    VARCHAR(255) NOT NULL,
    start                   DATETIME NOT NULL,
    end                     DATETIME NOT NULL,
    allowed_submissions     VARCHAR(255) NOT NULL,
    round                   VARCHAR(255) NOT NULL,
    type                    VARCHAR(255) NOT NULL,
    created                 DATETIME NOT NULL
);

CREATE TABLE teams (
    id              VARCHAR(255) NOT NULL PRIMARY KEY,
    owner           VARCHAR(255) NOT NULL,
    partner         VARCHAR(255) NOT NULL,
    competition_id  VARCHAR(255) NOT NULL,
    bot1            VARCHAR(255) NOT NULL,
    bot2            VARCHAR(255) NOT NULL,
    created         DATETIME NOT NULL
);

CREATE TABLE games_2v2 (
    id                      VARCHAR(255) NOT NULL PRIMARY KEY,
    competition_id          VARCHAR(255) NOT NULL,
    round                   VARCHAR(255) NOT NULL,
    team1_id                VARCHAR(255) NOT NULL,
    team2_id                VARCHAR(255) NOT NULL,
    team1bot1_id            VARCHAR(255) NOT NULL,
    team1bot2_id            VARCHAR(255) NOT NULL,
    team2bot1_id            VARCHAR(255) NOT NULL,
    team2bot2_id            VARCHAR(255) NOT NULL,
    team1bot1_survived      BOOLEAN NOT NULL,
    team1bot2_survived      BOOLEAN NOT NULL,
    team2bot1_survived      BOOLEAN NOT NULL,
    team2bot2_survived      BOOLEAN NOT NULL,
    log_file_path           VARCHAR(4096) NOT NULL,
    public                  BOOLEAN NOT NULL,
    additional_data         TEXT NOT NULL,
    created                 DATETIME NOT NULL
);

CREATE TABLE bots (
    id              VARCHAR(255) NOT NULL PRIMARY KEY,
    team_id         VARCHAR(255) NOT NULL,
    version         VARCHAR(255) NOT NULL,
    created         DATETIME NOT NULL
);