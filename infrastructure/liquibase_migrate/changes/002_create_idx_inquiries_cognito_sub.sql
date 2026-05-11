--liquibase formatted sql

--changeset 002_create_idx_inquiries_cognito_sub:002 context:main,develop
--validCheckSum ANY
-- Create index on cognito_sub for faster lookups
CREATE INDEX ASYNC idx_inquiries_cognito_sub ON inquiries(cognito_sub);

--changeset 002_create_idx_inquiries_cognito_sub:002-local context:local
-- Create index on cognito_sub for faster lookups (local PostgreSQL)
CREATE INDEX idx_inquiries_cognito_sub ON inquiries(cognito_sub);
