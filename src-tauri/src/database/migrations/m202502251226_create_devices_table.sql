CREATE TABLE IF NOT EXISTS "devices" (
	"id"	uuid_text NOT NULL,
	"host"	varchar NOT NULL,
	"port"	integer NOT NULL,
	"name"	varchar NOT NULL,
	"client_private_key" blob NOT NULL,
	"server_public_key"	blob,
	"order"	integer NOT NULL DEFAULT 0,
	"created_at"	datetime_text NOT NULL,
	PRIMARY KEY("id")
);