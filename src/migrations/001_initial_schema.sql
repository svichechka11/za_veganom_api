CREATE TABLE "products" (
	"id"	INTEGER NOT NULL UNIQUE GENERATED ALWAYS AS IDENTITY,
	"is_vegan"	TEXT NOT NULL,
	"name"	TEXT NOT NULL,
	"brand"	TEXT,
	"certification"	TEXT,
	"project_check"	TEXT NOT NULL,
	"ingredients"	TEXT,
	"photo"	    TEXT,
	"additional"	TEXT,
	"barcode"	TEXT UNIQUE,
	"status" TEXT NOT NULL DEFAULT 'active',
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	PRIMARY KEY("id")
);

CREATE TABLE "tags" (
	"id"	INTEGER NOT NULL UNIQUE GENERATED ALWAYS AS IDENTITY,
	"tag"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE TABLE "product_tags" (
	"product_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("product_id","tag_id"),
	FOREIGN KEY("product_id") REFERENCES "products"("id"),
	FOREIGN KEY("tag_id") REFERENCES "tags"("id")
);
