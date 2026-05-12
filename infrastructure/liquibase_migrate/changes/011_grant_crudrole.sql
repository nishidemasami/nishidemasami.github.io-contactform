--liquibase formatted sql

--changeset 011_grant_crudrole:011

-- Grant crudrole role
GRANT SELECT, INSERT, UPDATE, DELETE ON inquiries IN SCHEMA public TO crudrole;