-- Add migration script here
CREATE TABLE
    IF NOT EXISTS "event_dependency" (
        "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        "event_id" integer NOT NULL,
        "dependency_id" integer NOT NULL,
        FOREIGN KEY ("event_id") REFERENCES "event" ("id"),
        FOREIGN KEY ("dependency_id") REFERENCES "event" ("id")
    );