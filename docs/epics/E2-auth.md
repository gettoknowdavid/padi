# E2 — Authentication & Multi-Tenancy
**Depends on**: E1 complete
**Goal**: Implement secure authentication (email/password, phone OTP, Google OAuth, magic link) and multi-tenant organization management with role-based access control.

---

## Tickets

| ID | Title | Status |
|----|-------|--------|
| E2-T1 | User registration + email/password login | ⬜ |
| E2-T2 | Phone OTP login (Termii/AfricasTalking) | ⬜ |
| E2-T3 | Google OAuth + magic link | ⬜ |
| E2-T4 | JWT middleware + refresh token rotation | ⬜ |
| E2-T5 | Organization creation + invite flow + RBAC | ⬜ |

---

## E2-T1 — User Registration + Email/Password Login

**Goal**: Core auth: register with email, verify email, login, logout.

**Acceptance Criteria**:
- [ ] `POST /api/v1/auth/register` — creates user, sends verification email via Resend
- [ ] `POST /api/v1/auth/verify-email` — accepts token from email link, marks user verified
- [ ] `POST /api/v1/auth/login` — returns `{ access_token, refresh_token, user }`
- [ ] `POST /api/v1/auth/logout` — invalidates refresh token in Redis
- [ ] Password hashed with Argon2id (use `argon2` crate)
- [ ] Rate limit: 5 login attempts per 15 min per IP → 429
- [ ] Account lockout after 10 failed attempts in 1 hour → email notification
- [ ] Validation: email format, password min 8 chars, no common passwords
- [ ] Audit log entry on every login (success + failure)

**Endpoints**:
```
POST /api/v1/auth/register        { email, password, full_name, phone? }
POST /api/v1/auth/verify-email    { token }
POST /api/v1/auth/login           { email, password }
POST /api/v1/auth/logout          (requires auth header)
POST /api/v1/auth/forgot-password { email }
POST /api/v1/auth/reset-password  { token, new_password }
```

**Email templates** (Resend):
- Verification email: "Confirm your Padi account"
- Password reset email: "Reset your Padi password"

---

## E2-T2 — Phone OTP Login

**Goal**: Allow Nigerian users to sign up/login with phone number + OTP via SMS.

**Acceptance Criteria**:
- [ ] `POST /api/v1/auth/phone/send-otp` — sends 6-digit OTP via Termii (primary) with AfricasTalking fallback
- [ ] OTP stored in Redis with 10-minute TTL, max 3 verify attempts
- [ ] `POST /api/v1/auth/phone/verify-otp` — verifies OTP, returns tokens (creates user if first time)
- [ ] Phone normalized to E.164 before storing (`08012345678` → `+2348012345678`)
- [ ] Nigerian network prefix validation (warn but don't block non-Nigerian numbers)
- [ ] Rate limit: 3 OTP sends per phone per hour
- [ ] Audit log on OTP send and verify

**Endpoints**:
```
POST /api/v1/auth/phone/send-otp    { phone }
POST /api/v1/auth/phone/verify-otp  { phone, otp }
```

**OTP format**: 6 digits, numeric only. SMS text: "Your Padi verification code is: 123456. Valid for 10 minutes. Do not share."

**Termii integration**: Use `https://api.ng.termii.com/api/sms/otp/send` with token messaging type.

---

## E2-T3 — Google OAuth + Magic Link Login

**Goal**: Social login via Google and passwordless magic link via email.

**Acceptance Criteria**:

**Google OAuth**:
- [ ] `GET /api/v1/auth/google` — redirects to Google OAuth consent screen
- [ ] `GET /api/v1/auth/google/callback` — handles callback, creates/finds user, returns tokens
- [ ] Uses `oauth2` crate; no Passport.js equivalent needed in Rust
- [ ] Links Google account to existing user if email matches

**Magic Link**:
- [ ] `POST /api/v1/auth/magic-link` — generates signed token, sends email via Resend
- [ ] `GET /api/v1/auth/magic-link/verify?token=...` — verifies token (10-min TTL in Redis), returns auth tokens
- [ ] Token is a UUID stored in Redis; not a JWT (can be invalidated)

**Dependencies**:
```toml
oauth2 = "4"
```

---

## E2-T4 — JWT Middleware + Refresh Token Rotation

**Goal**: Secure token infrastructure — access + refresh token management, middleware for protected routes.

**Acceptance Criteria**:
- [ ] Access token: JWT, 15-minute expiry, signed with HS256, payload: `{ sub: user_id, org_id, role, iat, exp }`
- [ ] Refresh token: opaque UUID stored in Redis (key: `refresh:{token}`, value: `user_id`), 30-day expiry, rotated on every use
- [ ] `POST /api/v1/auth/refresh` — validates refresh token, issues new access + refresh token pair, invalidates old refresh token
- [ ] Auth middleware extracts Bearer token from `Authorization` header, validates JWT, injects `AuthUser` into request extensions
- [ ] `AuthUser` struct: `{ user_id, org_id, role }` — available in all protected handlers
- [ ] Expired access token returns 401 with `{ error: { code: "TOKEN_EXPIRED" } }` so clients know to refresh
- [ ] Compromised token detection: if a refresh token is used after rotation, revoke ALL tokens for that user (refresh token reuse attack)

**Files**:
```
backend/src/auth/
├── mod.rs
├── jwt.rs          # encode/decode, claims struct
├── middleware.rs   # Axum layer extracting AuthUser
└── tokens.rs       # refresh token CRUD in Redis
```

---

## E2-T5 — Organization Creation + Invite Flow + RBAC

**Goal**: Multi-tenant foundation — create organizations, invite team members, enforce role-based access.

**Acceptance Criteria**:

**Organization**:
- [ ] `POST /api/v1/organizations` — create org (authenticated user becomes Owner)
- [ ] `GET /api/v1/organizations/me` — list orgs the current user belongs to
- [ ] `PUT /api/v1/organizations/:id` — update org profile (Owner/Admin only)
- [ ] `GET /api/v1/organizations/:id/members` — list members with roles

**Invitations**:
- [ ] `POST /api/v1/organizations/:id/invitations` — generate invite link, send via email + optional WhatsApp link (body: `{ email, role }`)
- [ ] Invite token: UUID stored in DB (`invitations` table), 7-day expiry
- [ ] `GET /api/v1/invitations/:token` — preview invite (show org name, role)
- [ ] `POST /api/v1/invitations/:token/accept` — join org (creates member record)
- [ ] `DELETE /api/v1/organizations/:id/members/:user_id` — remove member (Owner only)
- [ ] `PUT /api/v1/organizations/:id/members/:user_id/role` — change role (Owner/Admin only)

**RBAC Middleware**:
- [ ] `require_role(min_role)` — extractor that checks `AuthUser.role` against minimum required role
- [ ] Role hierarchy: `Owner > Admin > Sales > Support > Agent`
- [ ] All org-scoped routes automatically scope queries to `AuthUser.org_id` — no manual filtering in handlers

**RBAC Matrix**:
| Action | Owner | Admin | Sales | Support | Agent |
|--------|-------|-------|-------|---------|-------|
| Manage members | ✅ | ✅ | ❌ | ❌ | ❌ |
| Edit org settings | ✅ | ✅ | ❌ | ❌ | ❌ |
| Create/edit contacts | ✅ | ✅ | ✅ | ✅ | Own only |
| Create/edit deals | ✅ | ✅ | ✅ | ❌ | Own only |
| View reports | ✅ | ✅ | Own | ❌ | ❌ |
| Create invoices | ✅ | ✅ | ✅ | ❌ | ❌ |
| Delete records | ✅ | ✅ | ❌ | ❌ | ❌ |

**New migration needed**: `invitations` table (id, org_id, email, role, token, invited_by, expires_at, accepted_at).
