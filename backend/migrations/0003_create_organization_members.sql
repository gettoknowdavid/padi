CREATE TABLE public.organization_members
(
    org_id          UUID        NOT NULL,
    user_id         UUID        NOT NULL,

    role            TEXT        NOT NULL,

    referral_code   TEXT UNIQUE NULL,
    commission_rate DECIMAL     NULL CHECK (commission_rate >= 0 AND commission_rate <= 1.0),

    invited_by      UUID        NULL REFERENCES public.users (id) ON DELETE SET NULL,
    invited_at      TIMESTAMPTZ NULL     DEFAULT NOW(),

    joined_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT organization_members_pkey PRIMARY KEY (org_id, user_id),
    CONSTRAINT organization_members_organization_id_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT organization_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users (id) ON DELETE RESTRICT
) TABLESPACE pg_default;

CREATE TRIGGER trigger_organization_members_updated_at
    BEFORE UPDATE
    ON public.organization_members
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();