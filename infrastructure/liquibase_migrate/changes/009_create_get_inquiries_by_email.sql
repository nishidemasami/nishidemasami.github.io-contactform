--liquibase formatted sql

--changeset 009_create_get_inquiries_by_email:009 splitStatements:false

-- Create function to get inquiries by email
CREATE FUNCTION get_inquiries_by_email(p_email VARCHAR(255))
RETURNS TABLE (
    id UUID,
    email VARCHAR(255),
    subject TEXT,
    body TEXT,
    created_at timestamptz
)
LANGUAGE SQL
AS $func$
    SELECT
        id,
        email,
        subject,
        body,
        created_at
    FROM inquiries
    WHERE email = p_email;
$func$;