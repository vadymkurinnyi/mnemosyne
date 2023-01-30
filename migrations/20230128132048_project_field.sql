-- Add migration script here
ALTER TABLE Projects
ADD COLUMN settings jsonb NOT NULL DEFAULT '{}';