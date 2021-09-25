#[derive(Debug)] // allows to debug issues

enum Platform {
    Windows,
    Linux,
    Macos,
}

impl Platform {
    fn parse(platform_arg: &str) -> Platform {
        if platform_arg == "windows" {
            Platform::Windows
        } else if platform_arg == "linux" {
            Platform::Linux
        } else if platform_arg == "macos" {
            Platform::Macos
        } else {
            panic!(
                "Unknown platform: {}. Valid values: windows, linux, macos",
                platform_arg
            );
        }
    }
}
