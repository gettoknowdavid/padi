CREATE TABLE public.tasks
(
    id               UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id           UUID        NOT NULL,

    title            TEXT        NOT NULL,
    description      TEXT        NULL,

    due_at           TIMESTAMPTZ NULL,
    priority         TEXT        NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high')),
    status           TEXT        NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'done')),
    linked_to_type   TEXT        NOT NULL CHECK (linked_to_type IN ('contact', 'deal', 'company')),
    linked_to_id     UUID        NOT NULL,

    assigned_to      UUID        NULL REFERENCES public.users (id) ON DELETE SET NULL,
    reminder_sent_at TIMESTAMPTZ NULL,

    created_by       UUID        NOT NULL REFERENCES public.users (id) ON DELETE SET NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ NULL,

    CONSTRAINT tasks_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;


CREATE TRIGGER trigger_tasks_updated_at
    BEFORE UPDATE
    ON public.tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();