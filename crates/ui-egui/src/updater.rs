pub fn check_latest_version(_v: &str) -> Option<(String, String)> {
    let url = "https://github.com/hassanve/gridcraft/releases";
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    Some((String::from("0.14.2"), String::from(url)))
}
