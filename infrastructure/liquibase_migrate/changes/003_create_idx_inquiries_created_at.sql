--liquibase formatted sql

--changeset 003_create_idx_inquiries_created_at:003
-- Create index on created_at for time-based queries
CREATE INDEX ASYNC idx_inquiries_created_at ON inquiries(created_at);