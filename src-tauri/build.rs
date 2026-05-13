use std::process::Command;

fn main() {
  tauri_build::build();

  #[cfg(target_os = "macos")]
  add_swift_runtime_rpaths();
}

#[cfg(target_os = "macos")]
fn add_swift_runtime_rpaths() {
  println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

  if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
    if output.status.success() {
      let developer_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();

      let swift_55 = format!(
        "{developer_dir}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
      );
      println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_55}");

      let swift_current =
        format!("{developer_dir}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
      println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_current}");
    }
  }
}
