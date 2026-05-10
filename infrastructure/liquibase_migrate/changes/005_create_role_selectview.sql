--liquibase formatted sql

--changeset 005_create_role_selectview:005

-- Create selectview role
CREATE ROLE selectview WITH LOGIN;