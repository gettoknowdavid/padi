CREATE TABLE public.audit_logs
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id        UUID        NOT NULL,
    user_id       UUID        NOT NULL,

    action        TEXT        NOT NULL,
    resource_type TEXT        NOT NULL,
    resource_id   UUID        NOT NULL,
    diff          JSONB       NULL,
    ip_address    TEXT        NULL,

    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT audit_logs_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT audit_logs_user_fkey FOREIGN KEY (user_id) REFERENCES public.users (id) ON DELETE RESTRICT
) TABLESPACE pg_default;