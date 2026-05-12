CREATE TABLE public.organizations
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    name              TEXT        NOT NULL,
    slug              TEXT        NOT NULL UNIQUE,
    logo_url          TEXT        NULL,

    address           TEXT,
    state_code        TEXT,
    lga               TEXT,

    cac_number        TEXT,
    tin               TEXT,

    phone             TEXT,
    whatsapp_number   TEXT,
    email             TEXT        NULL UNIQUE,

    subscription_plan TEXT        NOT NULL     DEFAULT 'free',
    sms_sender_id     TEXT        NULL,

    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at        TIMESTAMPTZ NULL
) TABLESPACE pg_default;

CREATE TRIGGER trigger_organizations_updated_at
    BEFORE UPDATE
    on public.organizations
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();