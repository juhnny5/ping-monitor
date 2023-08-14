use std::thread;
use std::time::{Duration, SystemTime};
use clap::{Arg, App};

pub mod ntfy;
fn main() {
    let matches = App::new("")
        .about("Ping Monitoring Service with alert via Ntfy.sh")
        .arg(Arg::with_name("ntfy_topic")
            .short("t")
            .long("topic")
            .value_name("NTFY_TOPIC")
            .help("Sets the ntfy topic")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("target_ips")
            .short("i")
            .long("ip")
            .value_name("TARGET_IP")
            .help("Sets the target IP addresses")
            .takes_value(true)
            .multiple(true)
            .required(true))
        .arg(Arg::with_name("timeout")
            .short("o")
            .long("timeout")
            .value_name("TIMEOUT_SECS")
            .help("Sets the timeout in seconds")
            .takes_value(true)
            .default_value("300"))
        .arg(Arg::with_name("interval")
            .short("v")
            .long("interval")
            .value_name("INTERVAL_SECS")
            .help("Sets the interval in seconds")
            .takes_value(true)
            .default_value("300"))
        .get_matches();

    let ntfy_topic = matches.value_of("ntfy_topic")
        .unwrap();

    let target_ips: Vec<&str> = matches.values_of("target_ips")
        .unwrap()
        .collect();
    
    let num_targets = target_ips.len();
    
    let timeout_secs = matches.value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .expect("Invalid timeout value");
    let interval_secs = matches.value_of("interval")
        .unwrap()
        .parse::<u64>()
        .expect("Invalid interval value");
    
    let mut target_statuses = vec!["up"; num_targets];
    let mut target_last_notification = vec![SystemTime::UNIX_EPOCH; num_targets];
    
    loop {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        for (i, &target_ip) in target_ips.iter().enumerate() {
            if let Ok(output) = std::process::Command::new("ping")
                .args(&["-c", "1", "-W", "1", target_ip])
                .output()
            {
                if output.status.success() {
                    if target_statuses[i] == "down" {
                        ntfy::notify(ntfy_topic, &format!("{} is up!", target_ip), "+1");
                        target_statuses[i] = "up";
                        target_last_notification[i] = SystemTime::now();
                    }
                } else {
                    let last_notification_time_secs = target_last_notification[i]
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs();

                    let time_diff = current_time - last_notification_time_secs;

                    if time_diff >= timeout_secs {
                        ntfy::notify(ntfy_topic, &format!("{} is down!", target_ip), "rotating_light");
                        target_last_notification[i] = SystemTime::now();
                    }
                    target_statuses[i] = "down";
                }
            }
        }

        thread::sleep(Duration::from_secs(interval_secs));
    }
}
