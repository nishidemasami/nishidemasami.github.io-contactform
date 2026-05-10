--liquibase formatted sql

--changeset 013_aws_iam_grant_select-function-lambda-role-develop:013

-- Grant selectview role to select-function-lambda-role-develop
AWS IAM GRANT selectview TO 'arn:aws:iam::779854054594:role/select-function-lambda-role-develop';