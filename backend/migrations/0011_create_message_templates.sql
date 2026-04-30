CREATE TABLE public.message_templates
(
    id                   UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id               UUID        NOT NULL,

    name                 TEXT        NOT NULL,
    channel              TEXT        NOT NULL CHECK (channel IN ('whatsapp', 'sms', 'email')),
    content              TEXT        NOT NULL,
    variables            TEXT[]      NULL     DEFAULT '{}',
    whatsapp_template_id TEXT        NULL,
    is_approved          BOOLEAN     NOT NULL DEFAULT FALSE,

    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ NULL,

    CONSTRAINT message_templates_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;


CREATE TRIGGER trigger_message_templates_updated_at
    BEFORE UPDATE
    ON public.message_templates
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();