-- Collection schema registry for RAKIT

CREATE TABLE IF NOT EXISTS collection_schemas (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection  TEXT NOT NULL UNIQUE,
    schema_json JSONB NOT NULL,
    version     INTEGER NOT NULL DEFAULT 1,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_collection_schemas_collection ON collection_schemas (collection);
