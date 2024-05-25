#!/bin/bash
#
# Copyright 2024 Anne Isabelle Macedo.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

export AWS_PAGER=""
aws sts get-caller-identity || exit 1
aws iam create-user --user-name github-ci

KEYS_CREATED=$(aws iam list-access-keys --user-name github-ci | jq '.AccessKeyMetadata')

if [ "$KEYS_CREATED" == "[]" ] ; then
    AKIA_OUTPUT=$(aws iam create-access-key --user-name github-ci)

    GEN_AWS_ACCESS_KEY_ID=$(echo $AKIA_OUTPUT | jq -r ".AccessKey.AccessKeyId")
    GEN_AWS_SECRET_ACCESS_KEY=$(echo $AKIA_OUTPUT | jq -r ".AccessKey.SecretAccessKey")
    gh secret set AWS_ACCESS_KEY_ID --body "$GEN_AWS_ACCESS_KEY_ID"
    gh secret set AWS_SECRET_ACCESS_KEY --body "$GEN_AWS_SECRET_ACCESS_KEY"
fi

aws s3api create-bucket --acl private --bucket retpolanne-ci-bucket
