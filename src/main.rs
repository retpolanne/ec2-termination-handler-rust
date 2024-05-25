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

mod prometheus;
mod imds;
use log::error;
use log::{info, trace};
use std::time;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::error::Error;
use signal_hook::flag;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::SignalsInfo;
use signal_hook::iterator::exfiltrator::WithOrigin;
use std::sync::atomic::AtomicBool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
    );
    info!("Started ec2 termination handler");

    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }
    let mut signals = SignalsInfo::<WithOrigin>::new(TERM_SIGNALS)?;

    tokio::spawn(async move {
        while !term_now.load(Ordering::Relaxed) {
            trace!("Checking if there's any spot events on the metadata");
            tokio::time::sleep(
                time::Duration::from_secs(2)
            ).await;
            let _ = imds::get_spot_itn_event()
                .await
                .and_then(
                    |ev|
                    if ev.is_some() {
                        info!("Received event notification {}", ev.unwrap().Action);
                        Ok(())
                    } else {
                        Ok(())
                    }
                )
                .map_err(|err| error!("Error checking for spot events: {}", err));
        }
        trace!("Stopped checking...");
    });

    for info in &mut signals {
        info!("Received signal {:?}", info.signal);
        match info.signal {
            _sigint => {
                info!("Gracefully shutting down ec2 termination handler");
                break;
            }
        }
    }

    prometheus::notify_termination_best_effort();
    info!("Last words from ec2 termination handler - bye");
    Ok(())
}
