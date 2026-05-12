--liquibase formatted sql

--changeset 004_create_users:004

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    cognito_sub UUID NOT NULL,
    email VARCHAR(255) NOT NULL,
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);
