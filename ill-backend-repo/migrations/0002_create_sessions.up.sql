-- Add up migration script here
create table sessions (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id),
    token text not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now()
);