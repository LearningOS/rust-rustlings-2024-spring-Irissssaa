// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



use std::process::Command;

fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let output = Command::new("cargo")
        .args(&["metadata", "--format-version=1"])
        .output()
        .expect("Failed to execute command");

    let metadata = String::from_utf8_lossy(&output.stdout);
    let metadata_json: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    
    let manifest_dir = metadata_json["workspace_root"].as_str().unwrap();

    println!("cargo:TEST_FOO={}", timestamp);
    println!("cargo:rerun-if-changed={}/build.rs", manifest_dir);

    // 添加特性控制代码
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
