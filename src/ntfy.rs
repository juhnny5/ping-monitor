/// Execute curl request to ntfy.sh service
pub fn notify(topic: &str, message: &str, tags: &str) {
    if let Ok(output) = std::process::Command::new("curl")
        .args(&[
            "-H", "Title: ping-monitor alert",
            "-H", "Priority: urgent",
            "-H", &format!("Tags: {}", tags),
            "-d", &format!("{}", message),
            &format!("ntfy.sh/{}", topic)
        ])
        .output()
    {
        if !output.status.success() {
            eprintln!("Failed to send notification: {:?}", output);
        }
    }
}
