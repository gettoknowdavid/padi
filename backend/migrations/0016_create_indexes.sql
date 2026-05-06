-- Organizations
CREATE INDEX idx_organizations_slug ON public.organizations (slug);

-- Users
CREATE INDEX idx_users_email ON public.users (email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_phone ON public.users (phone) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_created_at ON public.users (created_at);

-- Organization Members
CREATE INDEX idx_org_members_org_id ON public.organization_members (org_id);
CREATE INDEX idx_org_members_user_id ON public.organization_members (user_id);
CREATE INDEX idx_org_members_role ON public.organization_members (role);
CREATE UNIQUE INDEX idx_org_members_referral_code_unique ON public.organization_members (referral_code) WHERE referral_code IS NOT NULL;

-- Contacts
CREATE INDEX idx_contacts_org_id ON public.contacts (org_id);
CREATE INDEX idx_contacts_org_primary_phone ON public.contacts (org_id, primary_phone);
CREATE INDEX idx_contacts_org_whatsapp ON public.contacts (org_id, whatsapp_number);
CREATE INDEX idx_contacts_org_email ON public.contacts (org_id, email);
CREATE INDEX idx_contacts_created_by ON public.contacts (created_by);

-- Companies
CREATE INDEX idx_companies_org_id ON public.companies (org_id);
CREATE INDEX idx_companies_org_name ON public.companies (org_id, name);

-- Pipelines & Stages
CREATE INDEX idx_pipelines_org_id ON public.pipelines (org_id);
CREATE INDEX idx_pipeline_stages_pipeline_id ON public.pipeline_stages (pipeline_id);

-- Deals
CREATE INDEX idx_deals_org_id ON public.deals (org_id);
CREATE INDEX idx_deals_pipeline_id ON public.deals (pipeline_id);
CREATE INDEX idx_deals_stage_id ON public.deals (stage_id);
CREATE INDEX idx_deals_assigned_to ON public.deals (assigned_to);
CREATE INDEX idx_deals_status ON public.deals (status);

-- Tasks
CREATE INDEX idx_tasks_org_id ON public.tasks (org_id);
CREATE INDEX idx_tasks_linked_to ON public.tasks (linked_to_type, linked_to_id);
CREATE INDEX idx_tasks_assigned_to ON public.tasks (assigned_to);
CREATE INDEX idx_tasks_due_at ON public.tasks (due_at);

-- Communications
CREATE INDEX idx_communications_org_contact ON public.communications (org_id, contact_id);
CREATE INDEX idx_communications_external_id ON public.communications (external_message_id);

-- Invoices
CREATE INDEX idx_invoices_org_id ON public.invoices (org_id);
CREATE INDEX idx_invoices_contact_id ON public.invoices (contact_id);
CREATE INDEX idx_invoices_status ON public.invoices (status);
CREATE INDEX idx_invoices_created_by ON public.invoices (created_by);

-- Payments
CREATE INDEX idx_payments_org_id ON public.payments (org_id);
CREATE INDEX idx_payments_invoice_id ON public.payments (invoice_id);
CREATE INDEX idx_payments_gateway_reference ON public.payments (gateway_reference);

-- Audit Logs
CREATE INDEX idx_audit_logs_org_id ON public.audit_logs (org_id);
CREATE INDEX idx_audit_logs_user_id ON public.audit_logs (user_id);
CREATE INDEX idx_audit_logs_created_at ON public.audit_logs (created_at);