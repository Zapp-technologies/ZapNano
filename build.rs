use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("installerWindows/resources/images/ZapNano.ico");
        
        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile Windows resource (icon): {}", e);
        }
    }
}
