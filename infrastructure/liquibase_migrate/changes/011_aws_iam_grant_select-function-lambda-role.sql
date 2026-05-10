--liquibase formatted sql

--changeset 011_aws_iam_grant_select-function-lambda-role:011

-- Grant selectview role to select-function-lambda-role
AWS IAM GRANT selectview TO 'arn:aws:iam::779854054594:role/select-function-lambda-role';