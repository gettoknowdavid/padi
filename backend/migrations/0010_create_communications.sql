CREATE TABLE public.communications
(
    id                  UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id              UUID        NOT NULL,

    channel             TEXT        NOT NULL CHECK (channel IN ('whatsapp', 'sms', 'email', 'note')),
    direction           TEXT        NOT NULL CHECK (direction IN ('inbound', 'outbound')),

    contact_id          UUID        NOT NULL,
    content_preview     TEXT        NULL,
    full_content_url    TEXT        NULL,
    status              TEXT        NULL NULL CHECK (status IN ('delivered', 'failed', 'read', 'sent')),

    external_message_id TEXT        NULL,
    sent_by             UUID        NULL REFERENCES public.users (id) ON DELETE SET NULL,
    sent_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT communications_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT communications_contact_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_communications_updated_at
    BEFORE UPDATE
    ON public.communications
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();