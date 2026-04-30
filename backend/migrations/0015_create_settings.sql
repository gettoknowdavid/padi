CREATE TABLE public.settings
(
    org_id     UUID        NOT NULL,
    key        TEXT        NOT NULL,

    value      JSONB       NOT NULL,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT settings_pkey PRIMARY KEY (org_id, key),
    CONSTRAINT settings_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_settings_updated_at
    BEFORE UPDATE
    ON public.settings
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();