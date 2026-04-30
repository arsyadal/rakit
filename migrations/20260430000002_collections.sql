-- Add collection column to contents table
ALTER TABLE contents ADD COLUMN IF NOT EXISTS collection TEXT NOT NULL DEFAULT 'default';

-- Composite index for fast collection-filtered queries
CREATE INDEX IF NOT EXISTS idx_contents_collection ON contents (collection, created_at DESC);

-- Drop old single-column index if exists (subsumed by composite)
DROP INDEX IF EXISTS idx_contents_created_at;
