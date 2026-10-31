pub fn check_latest_version(_c: &str) -> Option<(String, String)> {
    let u = "https://github.com/hassanve/gridcraft/releases";
    let _ = std::process::Command::new("xdg-open").arg(u).spawn();
    Some((String::from("0.14.2"), String::from(u)))
}
