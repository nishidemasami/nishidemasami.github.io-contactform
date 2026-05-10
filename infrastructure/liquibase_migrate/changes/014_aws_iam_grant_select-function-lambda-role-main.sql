--liquibase formatted sql

--changeset 014_aws_iam_grant_select-function-lambda-role-main:014

-- Grant selectview role to select-function-lambda-role-main
AWS IAM GRANT selectview TO 'arn:aws:iam::779854054594:role/select-function-lambda-role-main';