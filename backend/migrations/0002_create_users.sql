CREATE TABLE public.users
(
    id                    UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    email                 TEXT UNIQUE NULL,
    phone                 TEXT UNIQUE NULL,

    full_name             TEXT        NULL,
    avatar_url            TEXT        NULL,

    password_hash         TEXT        NULL,

    is_verified           BOOLEAN     NOT NULL DEFAULT FALSE,
    failed_login_attempts INT         NOT NULL DEFAULT 0,
    locked_until          TIMESTAMPTZ,

    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at            TIMESTAMPTZ NULL

) TABLESPACE pg_default;

CREATE TRIGGER trigger_users_updated_at
    BEFORE UPDATE
    ON public.users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

