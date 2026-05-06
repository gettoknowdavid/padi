CREATE TABLE public.pipelines
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    org_id     UUID        NOT NULL,

    name       TEXT        NOT NULL,
    is_default BOOLEAN     NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT pipelines_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE
) TABLESPACE pg_default;

CREATE TRIGGER trigger_pipelines_updated_at
    BEFORE UPDATE
    ON public.pipelines
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE public.pipeline_stages
(
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    pipeline_id         UUID    NOT NULL,

    name                TEXT    NOT NULL,
    order_index         INTEGER NOT NULL,
    probability_default INTEGER NOT NULL DEFAULT 50 CHECK (probability_default BETWEEN 0 AND 100),

    CONSTRAINT pipeline_stages_pipeline_fkey FOREIGN KEY (pipeline_id) REFERENCES public.pipelines (id) ON DELETE CASCADE
);

