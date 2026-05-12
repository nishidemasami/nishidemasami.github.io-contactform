--liquibase formatted sql

--changeset 020_aws_iam_grant_crudrole-lambda-role-release:020 context:release

-- Grant crudrole role to crudrole-lambda-role-release
AWS IAM GRANT crudrole TO 'arn:aws:iam::672530906129:role/crudrole-lambda-role-release';
