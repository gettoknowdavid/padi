CREATE TABLE public.contact_companies
(
    contact_id      UUID NOT NULL,
    company_id      UUID NOT NULL,
    role_at_company TEXT NULL,

    CONSTRAINT contact_companies_pkey PRIMARY KEY (contact_id, company_id),
    CONSTRAINT contact_companies_contact_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts (id) ON DELETE CASCADE,
    CONSTRAINT contact_companies_company_fkey FOREIGN KEY (company_id) REFERENCES public.companies (id) ON DELETE CASCADE
) TABLESPACE pg_default;