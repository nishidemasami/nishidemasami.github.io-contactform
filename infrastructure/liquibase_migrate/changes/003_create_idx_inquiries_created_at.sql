--liquibase formatted sql

--changeset 003_create_idx_inquiries_created_at:003 context:main,develop
--validCheckSum ANY
-- Create index on created_at for time-based queries
CREATE INDEX ASYNC idx_inquiries_created_at ON inquiries(created_at);

--changeset 003_create_idx_inquiries_created_at:003-local context:local
-- Create index on created_at for time-based queries (local PostgreSQL)
CREATE INDEX idx_inquiries_created_at ON inquiries(created_at);
