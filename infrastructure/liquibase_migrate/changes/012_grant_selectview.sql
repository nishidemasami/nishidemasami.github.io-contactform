--liquibase formatted sql

--changeset 012_grant_selectview:012

-- Grant selectview role
GRANT SELECT ON inquiries IN SCHEMA public TO selectview;