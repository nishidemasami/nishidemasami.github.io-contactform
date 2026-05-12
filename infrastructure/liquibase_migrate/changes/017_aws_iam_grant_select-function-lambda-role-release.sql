--liquibase formatted sql

--changeset 017_aws_iam_grant_select-function-lambda-role-release:017 context:release

-- Grant selectview role to select-function-lambda-role-release
AWS IAM GRANT selectview TO 'arn:aws:iam::672530906129:role/select-function-lambda-role-release';
