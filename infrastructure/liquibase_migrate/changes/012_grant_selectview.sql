--liquibase formatted sql

--changeset 012_grant_selectview:012

-- Grant selectview role
GRANT SELECT ON public.inquiries TO selectview;