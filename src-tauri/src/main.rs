// Necessary imports
use std::fs;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

// Error handling structure
struct ScanError {
    message: String,
}

// Function to execute shell commands asynchronously
async fn exec_shell_command_async(
    cmd: &str,
    args: &str,
    output_file: &str,
) -> Result<(), ScanError> {
    let full_command = format!("{} {} > {}", args, cmd, output_file);
    let output = Command::new(cmd)
        .arg("-Command")
        .arg(&full_command)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .await?;

    if !output.status.success() {
        return Err(ScanError {
            message: format!("Error executing command: {:?}", output.status),
        });
    }

    Ok(())
}

// Function to scan the local system
async fn scan_local(folder_path: &PathBuf) -> Result<String, ScanError> {
    let commands = vec![
        // ... other commands ...
        (
            "WMIC",
            "WMIC Product get Caption, Version, Publisher",
            "wmic.txt",
        ),
        (
            "cmd",
            format!("echo Y | winget export -o {} 2>&1 >> {}", folder_path.join("wing.txt").display(), folder_path.join("wing_errs.txt").display()),
            "wing.txt",
        ),
    ];

    for (cmd, args, output_file) in commands {
        let output_path = folder_path.join(output_file);
        if let Err(e) = exec_shell_command_async(cmd, args, output_path).await {
            return Err(e);
        }
    }

    let combined_output = commands
        .iter()
        .map(|(cmd, args, output_file)| {
            let output = fs::read_to_string(folder_path.join(output_file)).unwrap();
            output
        })
        .collect::<Vec<String>>()
        .concat();

    let json_response = serde_json::to_string(&combined_output).unwrap();

    Ok(json_response)
}


