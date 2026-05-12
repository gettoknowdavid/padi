CREATE TABLE invitations
(
    id          UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    org_id      UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,

    email       TEXT        NOT NULL,
    role        TEXT        NOT NULL CHECK ( role IN ('owner', 'admin', 'sales', 'support', 'agent') ),

    token       TEXT UNIQUE NOT NULL,

    invited_by  UUID        NOT NULL REFERENCES users (id),
    expires_at  TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ NULL
);

CREATE TRIGGER trigger_invitations_updated_at
    BEFORE UPDATE
    ON public.invitations
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();