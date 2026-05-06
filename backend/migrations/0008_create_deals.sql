CREATE TABLE public.deals
(
    id                  UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id              UUID        NOT NULL,
    pipeline_id         UUID        NOT NULL,
    stage_id            UUID        NOT NULL,

    title               TEXT        NOT NULL,
    value_kobo          BIGINT      NOT NULL DEFAULT 0,
    currency            TEXT        NOT NULL DEFAULT 'NGN' CHECK (currency IN ('NGN', 'USD', 'EUR', 'GBP')),
    probability         INTEGER     NOT NULL DEFAULT 50 CHECK (probability BETWEEN 0 AND 100),
    expected_close_date DATE        NULL,
    won_lost_reason     TEXT        NULL,

    assigned_to         UUID        NULL REFERENCES public.users (id) ON DELETE SET NULL,
    created_by          UUID        NOT NULL REFERENCES public.users (id) ON DELETE SET NULL,

    status              TEXT        NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'won', 'lost')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ NULL,

    CONSTRAINT deals_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT deals_pipeline_fkey FOREIGN KEY (pipeline_id) REFERENCES public.pipelines (id) ON DELETE CASCADE,
    CONSTRAINT deals_stage_fkey FOREIGN KEY (stage_id) REFERENCES public.pipeline_stages (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_deals_updated_at
    BEFORE UPDATE
    ON public.deals
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE public.deal_contacts
(
    deal_id    UUID NOT NULL,
    contact_id UUID NOT NULL,

    CONSTRAINT deal_contacts_pkey PRIMARY KEY (deal_id, contact_id),
    CONSTRAINT deal_contacts_deal_fkey FOREIGN KEY (deal_id) REFERENCES public.deals (id) ON DELETE CASCADE,
    CONSTRAINT deal_contacts_contact_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts (id) ON DELETE CASCADE
);