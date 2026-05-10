--liquibase formatted sql

--changeset 012_grant_selectview:012

-- Grant selectview role
GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA public TO selectview;