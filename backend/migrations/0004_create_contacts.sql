CREATE TABLE public.contacts
(
    id               UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id           UUID        NOT NULL,

    full_name        TEXT        NOT NULL,
    primary_phone    TEXT        NOT NULL,
    secondary_phone  TEXT        NULL,
    whatsapp_number  TEXT        NOT NULL,
    email            TEXT        NULL,

    address          TEXT        NULL,
    state_code       TEXT        NOT NULL,
    lga              TEXT        NOT NULL,

    birthday         DATE        NULL,
    anniversary      DATE        NULL,

    custom_fields    JSONB       NULL     DEFAULT '{}'::JSONB,
    tags             TEXT[]      NULL,

    consent_given_at TIMESTAMPTZ NULL,
    consent_method   TEXT        NULL,

    created_by       UUID        NOT NULL REFERENCES public.users (id) ON DELETE CASCADE,
    updated_by       UUID        NULL REFERENCES public.users (id) ON DELETE SET NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at       TIMESTAMPTZ NULL,

    CONSTRAINT contacts_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_contacts_updated_at
    BEFORE UPDATE
    ON public.contacts
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();