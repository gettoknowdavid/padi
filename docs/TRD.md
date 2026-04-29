# Technical Requirements Document (TRD)
## Padi – CRM Platform Built for Nigerian Businesses
**Version**: 2.0
**Date**: 29 April 2026
**Author**: David Michael II
**Status**: Approved for Epic Breakdown
**Revision**: Expanded from v1.0 with Nigerian-context features, stack clarifications, and solo-dev constraints

---

## 1. Introduction

Padi is a full-featured, affordable Customer Relationship Management (CRM) platform purpose-built for Nigerian SMEs and growing businesses. The name "Padi" (Nigerian Pidgin for "friend/ally") reflects the product's core promise: a tool that feels like a trusted business partner, not enterprise software.

Padi replaces the WhatsApp + Excel + paper ledger workflows that dominate Nigerian SME operations with proper sales tracking, customer retention tools, instalment payment management, and deep integrations with Nigerian payment gateways and communication channels — while staying compliant with NDPR, priced in Naira, and designed for variable internet connectivity.

### 1.1 Problem Statement

Nigerian SMEs currently manage customer relationships through:
- WhatsApp groups and DMs (no history, no search, no handover when staff leave)
- Excel spreadsheets (manual, error-prone, not accessible on mobile)
- Paper ledgers (no backup, no analytics)
- Memory (a single point of failure)

Existing CRMs (Salesforce, HubSpot, Zoho) are priced in USD, designed for Western workflows, require stable broadband, and have no Nigerian payment or messaging integrations.

### 1.2 Target Users

- **Primary**: Nigerian SME owners (retail, services, FMCG distribution, fashion, food)
- **Secondary**: Sales reps, field agents, customer support staff at these businesses
- **Tertiary**: Small startups graduating from spreadsheets (5–50 employees)

---

## 2. Project Objectives

1. Deliver a CRM that Nigerian businesses actually adopt — affordable (priced in Naira), localized (Pidgin-aware copy), and mobile-first.
2. Reduce customer churn and improve repeat sales through lifecycle marketing and instalment payment tracking.
3. Provide seamless integration with Nigerian payment gateways (Paystack, Flutterwave), WhatsApp Business API, and SMS.
4. Ensure data sovereignty, NDPR compliance, and local trust signals.
5. Achieve a working MVP in 10–14 weeks as a solo developer using the Benn_X1 ticket-per-session workflow.
6. Prefer free-tier infrastructure during development and early beta; design for easy upgrade.

---

## 3. Scope

### 3.1 In Scope (MVP — v1.0)

- Multi-tenant user and organization management with RBAC
- Contact management with Nigerian phone normalization and deduplication
- Company/account management
- Deal pipeline with customizable stages
- Tasks, reminders, and a simple calendar view
- Communication hub: WhatsApp Business API message logging, SMS logging, Email logging
- Invoicing with Nigerian tax format (FIRS-compliant) and Paystack/Flutterwave checkout
- Instalment / hire-purchase payment plan tracking
- Basic dashboards and exportable reports (CSV/PDF)
- Mobile app (Flutter) with full offline support
- Web app (React/Next.js) — responsive, mobile-first
- NDPR compliance tools (consent management, data export, deletion requests)
- Agent/referral network tracking (basic)

### 3.2 Out of Scope (Phase 2+)

- Full accounting / ERP integration
- AI-powered analytics or forecasting
- USSD interface
- Multi-language beyond English + Pidgin copy adjustments
- Native desktop app
- Marketplace of third-party add-ons
- Advanced automation / workflow builder
- Bulk WhatsApp campaigns (requires WhatsApp approval process)

---

## 4. Functional Requirements

### 4.1 Authentication & Multi-Tenancy

- Sign-up via email/password, phone number + OTP (Termii/AfricasTalking), or Google OAuth
- Magic link login via email (Resend)
- Multi-tenant: each business is an isolated `organization` (workspace)
- A user may belong to multiple organizations with different roles in each
- Roles: `Owner`, `Admin`, `Sales`, `Support`, `Agent` (read-only + own contacts)
- Invite flow: email + WhatsApp invite link with expiry
- Session management: JWT access token (15-min expiry) + refresh token (30-day, rotating)
- Password reset via email and SMS OTP

### 4.2 Contact Management

- Fields: full name, primary phone (Nigerian format), secondary phone, email, WhatsApp number, physical address (state/LGA), business name, notes, tags, custom fields (per org)
- Phone validation and normalization: accept `08012345678`, `+2348012345678`, `2348012345678` — store in E.164
- Smart duplicate detection using phonetic name matching + phone number matching
- Activity timeline per contact: calls logged, deals linked, messages sent, payments received, notes added
- Bulk import via CSV with field mapping UI
- QR code contact capture card (for trade shows / market days)
- Contact segments / smart lists based on filter criteria (e.g., "customers who bought in last 30 days")
- Birthday and anniversary reminders with WhatsApp notification

### 4.3 Company / Account Management

- Company profiles linked to multiple contacts
- Industry, size, location, custom fields
- Deal and communication history at company level

### 4.4 Deal Pipeline

- Fully customizable pipeline stages per organization
- Deal fields: title, value (NGN or USD), probability %, expected close date, assigned rep, linked contacts/company
- Won/lost tracking with required reason field (for reporting)
- Deal activity log (stage changes, notes, tasks)
- Multiple pipelines per organization (e.g., New Sales vs. Renewals)

### 4.5 Tasks & Calendar

- Tasks linked to contacts, deals, or companies
- Due date, priority, assigned user, completion status
- Reminders via in-app notification, WhatsApp message, or SMS
- Simple calendar view (day/week/month) of tasks and scheduled follow-ups
- Recurring tasks

### 4.6 Communication Hub

- **WhatsApp**: Log inbound/outbound messages via WhatsApp Business Cloud API; send templated messages (approved templates); one-tap "Start WhatsApp chat" from any contact
- **SMS**: Send and log SMS via AfricasTalking or Termii; delivery status tracking
- **Email**: Log outbound emails; BCC-to-CRM forwarding address for logging inbound
- Unified inbox view across all channels per contact
- Message templates with variable substitution (contact name, deal value, due date, etc.)
- Template library per organization

### 4.7 Invoicing & Payments

- Create invoices in Nigerian format: seller details, buyer details, line items, VAT (7.5%), withholding tax option, invoice number (auto-sequential), issue date, due date
- Currency: NGN primary, USD secondary (stored as NGN equivalent at time of invoice)
- Send invoice link via WhatsApp, SMS, or email
- Paystack and Flutterwave payment links embedded in invoice
- Webhook-driven payment reconciliation: mark invoice as paid when payment received
- **Instalment / hire-purchase plans**: split invoice into scheduled partial payments, track each payment, send automatic reminders for upcoming instalments via WhatsApp
- Overdue invoice alerts (to sales rep and to customer via WhatsApp/SMS)
- Export invoices as PDF (FIRS-ready format)
- Subscription billing management (for Padi's own billing, not customer billing)

### 4.8 Agent / Referral Network Tracking

- Agents are users with the `Agent` role
- Each agent has a referral link / code
- Track which contacts and deals were originated by each agent
- Commission calculation (manual entry of rate %, auto-calculate owed amount)
- Agent performance report

### 4.9 Reporting & Analytics

- Sales funnel report (deals by stage, conversion rates)
- Revenue report (won deals by period, by rep, by pipeline)
- Customer retention / repeat purchase rate
- Invoice and payment summary (paid, overdue, outstanding)
- Agent performance report
- Export all reports to CSV and PDF
- Date range filters on all reports

### 4.10 Settings & Customization

- Organization profile (logo, business name, address, CAC number, TIN)
- Custom pipeline stages
- Custom contact fields
- WhatsApp Business number configuration
- SMS sender ID configuration
- Email forwarding address for BCC logging
- Team management (invite, role change, deactivate)
- Subscription plan and billing

### 4.11 NDPR Compliance

- Consent tracking per contact (when and how consent was obtained)
- Data subject rights: contact can request export of their data or deletion
- Deletion cascade: removing a contact removes all linked PII
- Audit log: who accessed/modified what, with timestamp (immutable, append-only)
- Data processing agreement template for organizations to share with their customers

---

## 5. Nigerian-Context Specific Requirements

These requirements distinguish Padi from generic CRMs and are non-negotiable for product-market fit:

### 5.1 Offline-First Mobile Experience

- All contact, deal, and task data accessible offline via local Isar database
- Changes made offline queued and synced when connectivity returns (conflict resolution: last-write-wins with server timestamp, surfaced to user for manual resolution on conflict)
- Sync status indicator always visible
- Low-bandwidth mode: compress API payloads, lazy-load images

### 5.2 Nigerian Phone Number Handling

- Accept and normalize all common Nigerian formats at every input field
- Validate against known NIN prefixes (0703, 0706, 0803, 0806, 0810, 0813, 0814, 0816, 0903, 0906, 0913, 0916 for MTN; etc.)
- Flag numbers that don't match any Nigerian network prefix (but don't block — international contacts exist)
- WhatsApp number defaults to primary phone if not specified separately

### 5.3 Currency & Money Display

- Always display NGN amounts as `₦5,000` (not `₦5,000.00`) in the UI — Nigerians do not use decimals for Naira
- Store all monetary values as integers (kobo) in the database
- Support NGN/USD deal values with exchange rate input (manual for now)
- Locale-aware thousands separator: `1,000,000` not `1000000`

### 5.4 WhatsApp as Primary Channel

- "Send via WhatsApp" button should be the most prominent action on contacts, deals, and invoices
- One-tap deep link: `https://wa.me/+234XXXXXXXXXX?text=...` with pre-filled template text
- WhatsApp Business API for template messages (automated reminders, payment confirmations)
- Log WhatsApp conversations initiated outside Padi via webhook

### 5.5 Invoice & Tax Compliance

- Invoice format matches FIRS requirements: TIN field, VAT breakdown, sequential invoice numbering
- Option to add WHT (Withholding Tax) deduction line
- Receipts generated on payment confirmation
- Proforma invoice support

### 5.6 Localized Copy

- All UI copy written in approachable Nigerian English (not stiff corporate English)
- Key actions use Pidgin-adjacent phrasing where appropriate: "Oya, add your first customer", "E don pay!", "Follow up sharp"
- Error messages are human — not "Record not found", but "We couldn't find that customer. Want to add them?"

---

## 6. Non-Functional Requirements

| Requirement | Target |
|---|---|
| API Response Time | < 300ms for 95th percentile |
| Concurrent Users | 1,000 per organization initially |
| Uptime | 99.9% (< 8.7 hours downtime/year) |
| Mobile Offline | Full functionality without internet |
| Data Residency | Africa region by default |
| Security | OWASP Top 10 compliant, E2E encryption for messages |
| Compliance | NDPR, FIRS invoice format |
| Accessibility | WCAG 2.1 AA |
| Bundle Size | Web initial load < 200KB gzipped |
| Mobile APK | < 30MB |

---

## 7. Technical Stack

### 7.1 Backend

| Component | Choice | Rationale |
|---|---|---|
| Language | Rust | Performance, safety, single binary deployment |
| Web Framework | Axum 0.7 | Async, ergonomic, well-maintained |
| Database | PostgreSQL 16 | Battle-tested, JSONB for flexible fields |
| ORM / Query | SQLx (compile-time checked) | Simpler than Diesel for solo dev; no dual-ORM overhead |
| Migrations | SQLx CLI (`sqlx migrate`) | Built-in to SQLx |
| Cache | Redis (Upstash free tier) | Sessions, rate limiting, job queue |
| Auth | Custom JWT (jsonwebtoken crate) | Full control; add Ory Kratos in Phase 2 if needed |
| Background Jobs | Tokio tasks + PostgreSQL-backed queue (pgmq or custom) | Avoid separate worker process on free tier |
| API Style | REST (OpenAPI 3.1 via utoipa) + WebSocket for real-time notifications |
| File Storage | Cloudflare R2 (free tier: 10GB) | S3-compatible, generous free tier |
| Email | Resend (free: 3,000/month) | Simple API, great deliverability |

### 7.2 Frontend — Web

| Component | Choice |
|---|---|
| Framework | Next.js 15 (App Router) |
| Language | TypeScript |
| Styling | TailwindCSS + shadcn/ui |
| State | Zustand (client) + TanStack Query (server state) |
| Forms | React Hook Form + Zod |
| Charts | Recharts |
| Build | Turborepo (monorepo) |

### 7.3 Frontend — Mobile

| Component | Choice |
|---|---|
| Framework | Flutter 3.x |
| State Management | Riverpod 2.x |
| Local Database | Isar (offline-first) |
| HTTP | Dio + Retrofit |
| Navigation | GoRouter |
| UI Components | Custom + flutter_shadcn_ui where applicable |

### 7.4 Infrastructure (Free-Tier First)

| Service | Free Tier | Upgrade Path |
|---|---|---|
| Backend hosting | Railway (free tier: $5 credit/month) or Render (free: 512MB RAM) | Railway Pro |
| Database | Supabase (free: 500MB PostgreSQL) | Supabase Pro or Neon |
| Redis | Upstash (free: 10,000 req/day) | Upstash Pay-as-you-go |
| File Storage | Cloudflare R2 (free: 10GB) | R2 Pay-as-you-go |
| CDN | Cloudflare (free) | — |
| Web hosting | Vercel (free) | Vercel Pro |
| CI/CD | GitHub Actions (free: 2,000 min/month) | — |
| Monitoring | Sentry (free: 5,000 errors/month) | Sentry Team |
| Logging | Axiom (free: 500MB/month) | Axiom Pro |

### 7.5 Third-Party APIs

| Service | Purpose | Cost |
|---|---|---|
| Paystack | Payment processing, webhooks | Free (% per transaction) |
| Flutterwave | Payment processing (alternative) | Free (% per transaction) |
| WhatsApp Business Cloud API | Messaging (Meta) | Free (1,000 service convos/month) |
| AfricasTalking | SMS | Pay-per-SMS (~₦4/SMS) |
| Termii | SMS alternative + OTP | Pay-per-use |
| Google OAuth | Social login | Free |

---

## 8. Data Model (Detailed)

```
organizations
  id, name, slug, logo_url, address, state, lga,
  cac_number, tin, phone, email, subscription_plan,
  whatsapp_number, sms_sender_id, created_at

users
  id, email, phone (E.164), full_name, avatar_url,
  password_hash, is_verified, created_at

organization_members
  org_id, user_id, role (owner/admin/sales/support/agent),
  referral_code (for agents), commission_rate,
  invited_by, joined_at

contacts
  id, org_id, full_name, primary_phone (E.164),
  secondary_phone, whatsapp_number, email,
  address, state_code, lga,
  birthday, anniversary,
  custom_fields (JSONB), tags (text[]),
  consent_given_at, consent_method,
  created_by, created_at, updated_at

companies
  id, org_id, name, industry, size_range,
  address, state_code, website, custom_fields (JSONB),
  created_at

contact_companies
  contact_id, company_id, role_at_company

pipelines
  id, org_id, name, is_default, created_at

pipeline_stages
  id, pipeline_id, name, order_index, probability_default

deals
  id, org_id, pipeline_id, stage_id, title,
  value_kobo, currency (NGN/USD), probability,
  expected_close_date, won_lost_reason,
  assigned_to (user_id), created_by,
  status (open/won/lost), created_at, updated_at

deal_contacts
  deal_id, contact_id

tasks
  id, org_id, title, description, due_at,
  priority (low/medium/high), status (pending/done),
  linked_to_type (contact/deal/company), linked_to_id,
  assigned_to, reminder_sent_at, created_by, created_at

communications
  id, org_id, channel (whatsapp/sms/email/note),
  direction (inbound/outbound),
  contact_id, content_preview, full_content_url (R2),
  status (sent/delivered/read/failed),
  external_message_id, sent_by, sent_at

message_templates
  id, org_id, name, channel, content, variables (text[]),
  whatsapp_template_id, is_approved, created_at

invoices
  id, org_id, invoice_number, contact_id, company_id,
  deal_id, status (draft/sent/paid/overdue/cancelled),
  issue_date, due_date, subtotal_kobo, vat_kobo,
  wht_kobo, total_kobo, currency,
  payment_gateway, payment_link, paid_at, created_by, created_at

invoice_line_items
  id, invoice_id, description, quantity, unit_price_kobo, amount_kobo

payment_plans
  id, invoice_id, total_instalments, frequency (weekly/monthly),
  created_at

payment_plan_instalments
  id, plan_id, instalment_number, amount_kobo, due_date,
  paid_at, payment_reference, status (pending/paid/overdue)

payments
  id, org_id, invoice_id, amount_kobo, gateway,
  gateway_reference, status, webhook_payload (JSONB),
  received_at

audit_logs
  id, org_id, user_id, action, resource_type, resource_id,
  diff (JSONB), ip_address, created_at

settings
  org_id, key, value (JSONB), updated_at
```

---

## 9. API Design Principles

- RESTful, versioned under `/api/v1/`
- OpenAPI 3.1 spec auto-generated via `utoipa`
- All responses: `{ data, meta, error }` envelope
- Pagination: cursor-based for large collections
- Auth: `Authorization: Bearer <jwt>` on all protected routes
- Rate limiting: 100 req/10s per user (Redis sliding window)
- Webhook signature verification for Paystack/Flutterwave
- WebSocket endpoint for real-time: `/ws` (notifications, sync events)

---

## 10. Security & Compliance

- All PII encrypted at rest (PostgreSQL encryption + Cloudflare encryption)
- TLS 1.3 in transit
- JWT access tokens: 15-min expiry; refresh tokens: 30-day rotating, stored httpOnly cookie
- CSRF protection on web
- SQL injection impossible via SQLx parameterized queries
- Input sanitization on all user-supplied data
- Rate limiting on auth endpoints (5 attempts/15min per IP)
- NDPR: consent logging, data export endpoint, deletion API with cascade, audit log
- SOC2-ready audit log from day one (append-only, no deletions)
- Webhook secrets validated with HMAC-SHA256

---

## 11. Folder Structure

```
padi/
├── backend/                    # Rust / Axum
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── db.rs
│   │   ├── auth/
│   │   ├── modules/
│   │   │   ├── contacts/
│   │   │   ├── deals/
│   │   │   ├── tasks/
│   │   │   ├── communications/
│   │   │   ├── invoices/
│   │   │   ├── payments/
│   │   │   ├── reports/
│   │   │   └── settings/
│   │   ├── integrations/
│   │   │   ├── paystack.rs
│   │   │   ├── flutterwave.rs
│   │   │   ├── whatsapp.rs
│   │   │   ├── africas_talking.rs
│   │   │   └── resend.rs
│   │   ├── jobs/              # Background tasks
│   │   ├── middleware/
│   │   └── utils/
│   ├── migrations/
│   └── Cargo.toml
│
├── web/                        # Next.js 15
│   ├── app/
│   │   ├── (auth)/
│   │   ├── (dashboard)/
│   │   │   ├── contacts/
│   │   │   ├── deals/
│   │   │   ├── tasks/
│   │   │   ├── inbox/
│   │   │   ├── invoices/
│   │   │   └── reports/
│   │   └── api/               # Next.js API routes (thin proxy or remove)
│   ├── components/
│   │   ├── ui/                # shadcn/ui components
│   │   └── features/
│   ├── lib/
│   │   ├── api.ts
│   │   └── utils.ts
│   └── package.json
│
├── mobile/                     # Flutter
│   ├── lib/
│   │   ├── main.dart
│   │   ├── core/
│   │   │   ├── auth/
│   │   │   ├── db/            # Isar schemas
│   │   │   └── sync/
│   │   ├── features/
│   │   │   ├── contacts/
│   │   │   ├── deals/
│   │   │   ├── tasks/
│   │   │   ├── inbox/
│   │   │   └── invoices/
│   │   └── shared/
│   └── pubspec.yaml
│
├── docs/
│   ├── TRD.md                 # This file
│   ├── epics/
│   └── tickets/
└── docker-compose.yml
```

---

## 12. Testing Strategy

- **Backend**: Unit tests for business logic (Rust `#[cfg(test)]`); integration tests against a test PostgreSQL instance spun up via Docker in CI
- **Web**: Unit tests for utilities (Vitest); component tests (Testing Library); E2E (Playwright — free)
- **Mobile**: Widget tests (Flutter test); integration tests (integration_test package)
- **API contract**: OpenAPI spec validated against actual responses in CI
- **Coverage target**: 70% on backend business logic; not obsessing over UI coverage for MVP

---

## 13. Assumptions & Dependencies

- Paystack and Flutterwave merchant accounts set up before payment module testing
- WhatsApp Business API access approved by Meta (takes 1–7 days)
- AfricasTalking or Termii account registered with sender ID approved (takes 3–5 days for Nigeria)
- Developer has a Nigerian phone number for testing OTP flows
- Domain registered and pointed to Cloudflare

---

## 14. Success Metrics (MVP)

| Metric | Target |
|---|---|
| Beta organizations onboarded | 50 within 30 days of launch |
| User preference vs. WhatsApp/Excel | > 80% in onboarding survey |
| Month-1 churn | < 5% |
| API p95 response time | < 300ms |
| Mobile crash rate | < 0.5% |
| Invoices paid via Padi link | > 60% of sent invoices |

---

## 15. Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| WhatsApp API approval delayed | Medium | High | Build SMS path first; WhatsApp as enhancement |
| Free-tier DB limits hit during beta | Low | Medium | Supabase 500MB is ~2M rows; sufficient for 50 orgs |
| Solo developer burnout | High | High | Strict ticket scope; no feature creep during sprints |
| Paystack webhook failures | Low | High | Idempotent webhook handler + manual reconciliation UI |
| NDPR audit | Low | High | Compliance built-in from day one, not retrofitted |

---

*This TRD is the single source of truth. Epics and tickets are derived from this document and must be updated if scope changes.*
