--liquibase formatted sql

--changeset 006_create_role_crudrole:006

-- Create crudrole role
CREATE ROLE crudrole WITH LOGIN;