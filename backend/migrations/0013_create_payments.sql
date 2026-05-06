-- PAYMENT_PLANS
CREATE TABLE public.payment_plans
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    invoice_id        UUID        NOT NULL,
    total_instalments INTEGER     NOT NULL,
    frequency         TEXT        NOT NULL CHECK (frequency IN ('weekly', 'monthly')),

    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT payment_plans_invoice_fkey FOREIGN KEY (invoice_id) REFERENCES public.invoices (id) ON DELETE CASCADE
) TABLESPACE pg_default;

-- PAYMENT_PLANS INSTALMENTS
CREATE TABLE public.payment_plan_instalments
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    plan_id           UUID        NOT NULL,
    instalment_number INTEGER     NOT NULL,
    amount_kobo       BIGINT      NOT NULL,
    due_date          DATE        NOT NULL,
    paid_at           TIMESTAMPTZ NULL,
    payment_reference TEXT        NULL,
    status            TEXT        NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'overdue')),

    CONSTRAINT payment_plan_instalments_plan_fkey FOREIGN KEY (plan_id) REFERENCES public.payment_plans (id) ON DELETE CASCADE
) TABLESPACE pg_default;

-- PAYMENTS
CREATE TABLE public.payments
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    org_id            UUID        NOT NULL,
    invoice_id        UUID        NOT NULL,
    amount_kobo       BIGINT      NOT NULL,
    gateway           TEXT        NOT NULL,
    gateway_reference TEXT        NOT NULL,
    status            TEXT        NOT NULL,
    webhook_payload   JSONB       NULL,
    received_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT payments_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT payments_invoice_fkey FOREIGN KEY (invoice_id) REFERENCES public.invoices (id) ON DELETE CASCADE
) TABLESPACE pg_default;
