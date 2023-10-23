-- Add migration script here
-- create table with columns for planned event
CREATE TABLE
    IF NOT EXISTS "planned_event" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "project_id" integer NOT NULL,
        "case_id" integer NOT NULL,
        "event_id" integer NULL,
        "activity" text NOT NULL,
        "description" text NULL,
        "earliest_start_date" bigint NOT NULL,
        "due_date" bigint NOT NULL,
        "completed" boolean NOT NULL,
        FOREIGN KEY ("event_id") REFERENCES "event" ("id"),
        FOREIGN KEY ("project_id") REFERENCES "project" ("id"),
        FOREIGN KEY ("case_id") REFERENCES "case" ("id")
    );