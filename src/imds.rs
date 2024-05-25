/*
 * Copyright 2024 Anne Isabelle Macedo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};
use aws_config::imds::client::Client;

#[derive(Debug, Clone)]
pub struct IMDSError;

impl Error for IMDSError {}

impl fmt::Display for IMDSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error calling instance metadata service")
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct InstanceAction {
    pub Action: String,
    pub Time: String
}

pub async fn get_spot_itn_event() -> Result<Option<InstanceAction>, IMDSError> {
    let imds_client = Client::builder().build();
    let instance_action = imds_client
        .get("/latest/meta-data/spot/instance-action".to_string())
        .await
        .map_err(|_err| IMDSError.into())?;
    let action : InstanceAction = serde_json::from_str(instance_action.as_ref())
        .map_err(|_err| IMDSError.into())?;
    Ok(Some(action))
}
