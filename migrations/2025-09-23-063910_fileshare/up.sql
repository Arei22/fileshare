create table public.uploads (
  uuid uuid primary key not null default gen_random_uuid(),
  expiration bigint not null,
  getted boolean not null
);
