-- RBAC tables for RAKIT

CREATE TABLE IF NOT EXISTS roles (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name       TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS permissions (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    action     TEXT NOT NULL,
    collection TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(action, collection)
);

CREATE TABLE IF NOT EXISTS role_permissions (
    role_id       UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id  UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- Seed default roles
INSERT INTO roles (name)
VALUES ('admin'), ('editor'), ('viewer'), ('public')
ON CONFLICT (name) DO NOTHING;

-- Seed permissions (action x collection wildcard)
INSERT INTO permissions (action, collection)
VALUES
    ('read', '*'),
    ('create', '*'),
    ('update', '*'),
    ('delete', '*')
ON CONFLICT (action, collection) DO NOTHING;

-- Map roles to permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
JOIN permissions p ON (
    (r.name = 'admin') OR
    (r.name = 'editor' AND p.action IN ('read', 'create', 'update')) OR
    (r.name = 'viewer' AND p.action = 'read') OR
    (r.name = 'public' AND p.action = 'read')
)
WHERE p.collection = '*'
ON CONFLICT DO NOTHING;
