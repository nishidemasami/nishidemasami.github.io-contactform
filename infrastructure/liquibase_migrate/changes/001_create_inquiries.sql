--liquibase formatted sql

--changeset 001_create_inquiries:001

-- Create inquiry table
CREATE TABLE inquiries (
    id UUID PRIMARY KEY,
    cognito_sub UUID NOT NULL,
    email VARCHAR(255) NOT NULL,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    reply TEXT,
    respondent TEXT,
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reply_at timestamptz
);
