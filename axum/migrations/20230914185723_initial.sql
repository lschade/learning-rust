-- Add migration script here
CREATE TABLE
    IF NOT EXISTS "project" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "name" text NOT NULL,
        "description" text NULL
    );

CREATE TABLE
    IF NOT EXISTS "case" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "project_id" integer NOT NULL,
        "name" text NOT NULL,
        "description" text NULL,
        FOREIGN KEY ("project_id") REFERENCES "project" ("id")
    );

CREATE TABLE
    IF NOT EXISTS "event" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "project_id" integer NOT NULL,
        "case_id" integer NOT NULL,
        "activity" text NOT NULL,
        "start_date" bigint NOT NULL,
        "end_date" bigint NULL,
        "location" text NULL,
        FOREIGN KEY ("project_id") REFERENCES "project" ("id"),
        FOREIGN KEY ("case_id") REFERENCES "case" ("id")
    );