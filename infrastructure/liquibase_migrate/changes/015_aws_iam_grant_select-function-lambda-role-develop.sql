--liquibase formatted sql

--changeset 015_aws_iam_grant_select-function-lambda-role-develop:015

-- Grant selectview role to select-function-lambda-role-develop
AWS IAM GRANT selectview TO 'arn:aws:iam::672530906129:role/select-function-lambda-role-develop';