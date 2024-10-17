use std::env;
use std::process::Command;

fn main() {
    #[cfg(target_os = "macos")]
    {
        let vulkan_sdk = env::current_dir()
            .unwrap()
            .join("../../../../../../VulkanSDK/MacOS/lib");

        let cmd = "@executable_path/".to_string() + vulkan_sdk.to_str().unwrap();

        Command::new("install_name_tool")
            .arg("-add_rpath")
            .arg(cmd)
            .arg("intersect/target/debug/examples/gpu")
            .status()
            .expect("Failed to add rpath to gpu example");

        println!("cargo:rerun-if-changed=build.rs");
    }
}
