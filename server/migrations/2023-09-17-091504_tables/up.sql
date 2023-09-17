-- Your SQL goes here

CREATE TABLE "channels" (
  "name" varchar(100) PRIMARY KEY,
  "token" varchar(400) NOT NULL,
  "title" varchar(100) NOT NULL,
  "owner" varchar(100) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "comments" (
  "id" integer PRIMARY KEY,
  "body" varchar(400) NOT NULL,
  "channel" varchar(100) NOT NULL,
  "owner" varchar(100) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);
