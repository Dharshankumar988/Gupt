-- 002_add_password.sql
-- Adds password hash column to users table for standard login

ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255) NOT NULL DEFAULT '';
