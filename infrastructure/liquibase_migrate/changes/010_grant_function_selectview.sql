--liquibase formatted sql

--changeset 010_grant_function_selectview:010

-- Grant execute permission on the function to selectview role
GRANT EXECUTE ON FUNCTION public.get_inquiries_by_email(VARCHAR(255)) TO selectview;
