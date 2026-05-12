--liquibase formatted sql

--changeset 019_aws_iam_grant_crudrole-lambda-role-main:019 context:main

-- Grant crudrole role to crudrole-lambda-role-main
AWS IAM GRANT crudrole TO 'arn:aws:iam::672530906129:role/crudrole-lambda-role-main';
