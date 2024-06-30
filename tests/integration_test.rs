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

// We want to ignore this test locally

use aws_sdk_ec2 as ec2;
use aws_sdk_ec2::types as ec2_types;
use aws_sdk_ec2::client::Waiters;
use aws_config::BehaviorVersion;
use std::{error::Error, fs::File, io::Read, io};
use std::time::Duration;
use base64::prelude::*;
use handlebars::Handlebars;
use log::{info, error};

async fn setup_ec2() -> Result<ec2::Client, ec2::Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

    Ok(ec2::Client::new(&config))
}

fn get_cloudinit(file_path: String) -> Result<String, io::Error> {
    let mut cloudinit_file = File::open(file_path)?;
    let mut cloudinit_contents = String::new();
    cloudinit_file.read_to_string(&mut cloudinit_contents)?;
    Ok(BASE64_STANDARD.encode(cloudinit_contents.as_bytes()))
}

#[tokio::test]
#[ignore]
async fn test_e2e() -> Result<(), Box<dyn Error>>{
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
    );
    let client_ec2 = setup_ec2().await.unwrap();

    info!("Creating monitor instance on AWS");
    let run_instances_monitor = client_ec2
        .run_instances()
        // us-east-1 Jammy Jellyfish amd64
        .image_id("ami-012485deee5681dc0".to_string())
        .instance_type(ec2_types::InstanceType::T3Micro)
        .user_data(get_cloudinit("resources/cloud-init-monitor.yml".to_string())?)
        .min_count(1)
        .max_count(1)
        .security_groups("github-ci")
        .tag_specifications(
            ec2_types::TagSpecification::builder()
                .resource_type(ec2_types::ResourceType::Instance)
                .tags(ec2_types::Tag::builder().key("Name".to_string()).value("Monitor".to_string()).build())
                .build()
        )
        .send()
        .await?;
    let monitor_instance_id = run_instances_monitor
        .instances()[0]
        .instance_id
        .as_ref();
    info!("Creating test instance on AWS");
    let run_instances = client_ec2
        .run_instances()
        // us-east-1 Jammy Jellyfish amd64
        .image_id("ami-012485deee5681dc0".to_string())
        .instance_type(ec2::types::InstanceType::T3Micro)
        .user_data(get_cloudinit("resources/cloud-init-test.yml".to_string())?)
        .min_count(1)
        .max_count(1)
        .tag_specifications(
            ec2_types::TagSpecification::builder()
                .resource_type(ec2_types::ResourceType::Instance)
                .tags(ec2_types::Tag::builder().key("Name".to_string()).value("Test-instance".to_string()).build())
                .build()
        )
        .send()
        .await?;
    let test_instance_id = run_instances
        .instances()[0]
        .instance_id
        .as_ref();
    info!("Will wait for monitor instance {} to be up", monitor_instance_id.unwrap());
    let _ = client_ec2.wait_until_instance_running()
        .instance_ids(monitor_instance_id.unwrap())
        .wait(Duration::from_secs(120))
        .await;
    info!("Will wait for test instance {} to be up", test_instance_id.unwrap());
    let _ = client_ec2.wait_until_instance_running()
        .instance_ids(test_instance_id.unwrap())
        .wait(Duration::from_secs(120))
        .await;
    info!("Will terminate test instance {}", test_instance_id.unwrap());
    client_ec2.terminate_instances()
              .instance_ids(test_instance_id.unwrap())
              .send()
              .await?;
    /*
    info!("Will terminate monitor instance {}", monitor_instance_id.unwrap());
    client_ec2.terminate_instances()
              .instance_ids(monitor_instance_id.unwrap())
              .send()
              .await?;
    */
    Ok(())
}
