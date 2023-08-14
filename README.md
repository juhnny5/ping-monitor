# Ping Monitor

This Rust tool monitors the availability of a list of target IP addresses and sends notifications to a specified topic using the ntfy.sh service.

The tool is designed to run in the background and continuously check the status of the target IPs. It provides the following features:

- Monitors multiple target IP addresses for their availability.
- Sends notifications when a target IP becomes unreachable or recovers.
- Configurable notification interval and timeout duration.
- Supports custom priority and tags for notifications.

### Usage

```
Ping Monitoring Service with alert via Ntfy.sh

USAGE:
    ping-monitor [OPTIONS] --topic <NTFY_TOPIC> --ip <TARGET_IP>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v, --interval <INTERVAL_SECS>    Sets the interval in seconds [default: 300]
    -t, --topic <NTFY_TOPIC>          Sets the ntfy topic
    -i, --ip <TARGET_IP>...           Sets the target IP addresses
    -o, --timeout <TIMEOUT_SECS>      Sets the timeout in seconds [default: 300]
```

### Example

```bash
ping-monitor --topic myNtfyTopic --ip 10.0.0.1 10.0.0.2 10.0.0.3
```
