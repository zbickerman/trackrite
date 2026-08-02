CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email               TEXT UNIQUE NOT NULL,
    password_hash       TEXT NOT NULL,
    display_name        TEXT,
    daily_calorie_goal  INTEGER,
    protein_goal_g      INTEGER,
    carb_goal_g         INTEGER,
    fat_goal_g          INTEGER,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);