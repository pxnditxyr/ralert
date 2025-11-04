-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  name TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT 'user',
  status TEXT NOT NULL DEFAULT 'active',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);


CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

INSERT INTO users (id, email, password, name, role, status, created_at, updated_at)
VALUES (
  '2534f460-9a6e-488e-81aa-06e8d9fc2b24',
  'admin@ralert.com',
  '$2b$10$LsUwrzcV4.X2Vxy9S1p6HO7Nuy9lJcxw9N3AI0xp.KhOkpn7fZoIK',
  'Pxndxs',
  'admin',
  'active',
  strftime('%s', 'now'),
  strftime('%s', 'now')
), (
  '5695974d-0b7a-4ae9-85ff-3b4f87bbf21f',
  'user@ralert.com',
  '$2b$10$LsUwrzcV4.X2Vxy9S1p6HO7Nuy9lJcxw9N3AI0xp.KhOkpn7fZoIK',
  'User',
  'user',
  'active',
  strftime('%s', 'now'),
  strftime('%s', 'now')
);
