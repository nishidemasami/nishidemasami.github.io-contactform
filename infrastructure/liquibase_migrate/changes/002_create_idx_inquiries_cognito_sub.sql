--liquibase formatted sql

--changeset 002_create_idx_inquiries_cognito_sub:002
-- Create index on cognito_sub for faster lookups
CREATE INDEX ASYNC idx_inquiries_cognito_sub ON inquiries(cognito_sub);
