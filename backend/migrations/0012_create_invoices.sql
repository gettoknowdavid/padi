CREATE TABLE public.invoices
(
    id              UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    org_id          UUID        NOT NULL,
    invoice_number  TEXT        NOT NULL,
    contact_id      UUID        NULL,
    company_id      UUID        NULL,
    deal_id         UUID        NULL,
    status          TEXT        NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'sent', 'paid', 'overdue', 'cancelled')),
    issue_date      DATE        NOT NULL DEFAULT CURRENT_DATE,
    due_date        DATE        NOT NULL,
    subtotal_kobo   BIGINT      NOT NULL DEFAULT 0,
    vat_kobo        BIGINT      NOT NULL DEFAULT 0,
    wht_kobo        BIGINT      NOT NULL DEFAULT 0,
    total_kobo      BIGINT      NOT NULL DEFAULT 0,
    currency        TEXT        NOT NULL DEFAULT 'NGN',
    payment_gateway TEXT        NULL,
    payment_link    TEXT        NULL,
    paid_at         TIMESTAMPTZ NULL,

    created_by      UUID        NOT NULL REFERENCES public.users (id) ON DELETE SET NULL,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ NULL,

    CONSTRAINT invoices_org_fkey FOREIGN KEY (org_id) REFERENCES public.organizations (id) ON DELETE CASCADE,
    CONSTRAINT invoices_contact_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts (id) ON DELETE SET NULL,
    CONSTRAINT invoices_company_fkey FOREIGN KEY (company_id) REFERENCES public.companies (id) ON DELETE SET NULL,
    CONSTRAINT invoices_deal_fkey FOREIGN KEY (deal_id) REFERENCES public.deals (id) ON DELETE SET NULL
) TABLESPACE pg_default;


CREATE TRIGGER trigger_invoices_updated_at
    BEFORE UPDATE
    ON public.invoices
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE public.invoice_line_items
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id      UUID    NOT NULL,
    description     TEXT    NOT NULL,
    quantity        INTEGER NOT NULL DEFAULT 1,
    unit_price_kobo BIGINT  NOT NULL,
    amount_kobo     BIGINT  NOT NULL,

    CONSTRAINT invoice_line_items_invoice_fkey FOREIGN KEY (invoice_id) REFERENCES public.invoices (id) ON DELETE CASCADE
);