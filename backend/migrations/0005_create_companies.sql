CREATE TABLE public.companies
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    org_id        UUID        NOT NULL,

    name          TEXT        NOT NULL,
    industry      TEXT        NULL,
    size_range    TEXT        NULL,

    address       TEXT        NULL,
    state_code    TEXT        NULL,

    website       TEXT        NULL,
    custom_fields JSONB       NULL     DEFAULT '{}'::JSONB,

    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ NULL,

    CONSTRAINT companies_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_companies_updated_at
    BEFORE UPDATE
    ON public.companies
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();