CREATE TABLE IF NOT EXISTS "devices" (
	"id"	uuid_text NOT NULL,
	"host"	varchar NOT NULL,
	"port"	integer NOT NULL,
	"name"	varchar NOT NULL,
	"access_token"	varchar,
	"order"	integer NOT NULL DEFAULT 0,
	"created_at"	datetime_text NOT NULL,
	PRIMARY KEY("id")
);