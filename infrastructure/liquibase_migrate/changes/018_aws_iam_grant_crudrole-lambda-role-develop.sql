--liquibase formatted sql

--changeset 018_aws_iam_grant_crudrole-lambda-role-develop:018 context:develop

-- Grant crudrole role to crudrole-lambda-role-develop
AWS IAM GRANT crudrole TO 'arn:aws:iam::672530906129:role/crudrole-lambda-role-develop';
