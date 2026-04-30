-- Initial schema for RAKIT
-- The core idea: a generic `contents` table backed by jsonb,
-- so the engine can handle dynamic content types without schema migrations.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS contents (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    data        JSONB NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- GIN index for fast jsonb querying
CREATE INDEX IF NOT EXISTS idx_contents_data_gin ON contents USING GIN (data);
CREATE INDEX IF NOT EXISTS idx_contents_created_at ON contents (created_at DESC);

-- Trigger to bump updated_at
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_contents_updated_at ON contents;
CREATE TRIGGER trg_contents_updated_at
    BEFORE UPDATE ON contents
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
