-- Add up migration script here
create table users (
    id uuid primary key default gen_random_uuid(),
    discord_id text not null,
    name text not null,
    image text not null,
    gd_id text,
    gd_name text,
    created_at timestamptz not null default now()
);