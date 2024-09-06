const COMMANDS: &[&str] = &[
    "send_analytics_alias",
    "send_analytics_group",
    "send_analytics_identify",
    "send_analytics_page",
    "send_analytics_screen",
    "send_analytics_track",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
