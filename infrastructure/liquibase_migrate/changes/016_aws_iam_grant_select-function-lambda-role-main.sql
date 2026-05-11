--liquibase formatted sql

--changeset 016_aws_iam_grant_select-function-lambda-role-main:016

-- Grant selectview role to select-function-lambda-role-main
AWS IAM GRANT selectview TO 'arn:aws:iam::672530906129:role/select-function-lambda-role-main';