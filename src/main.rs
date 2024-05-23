use std::io::Write;
use std::process::Command;
use structopt::StructOpt;
use tempfile::NamedTempFile;

/// CLI arguments struct
#[derive(StructOpt, Debug)]
#[structopt(name = "rosbag2_mcap_to_sqlite3")]
struct Cli {
    /// Input bag file
    input_bag: String,
    /// Output bag file
    output_bag: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    // Create a temporary YAML file
    let mut yaml_file = NamedTempFile::new().expect("Failed to create temporary file");

    // Write the YAML content to the temporary file
    write!(
        yaml_file,
        "output_bags:\n  - uri: {}\n    storage_id: sqlite3\n    all: true\n",
        args.output_bag
    )
    .expect("Failed to write to temporary file");

    // Get the path of the temporary file
    let yaml_file_path = yaml_file.path().to_str().expect("Invalid file path");

    // Run the conversion command
    let status = Command::new("ros2")
        .arg("bag")
        .arg("convert")
        .arg("-i")
        .arg(&args.input_bag)
        .arg("-o")
        .arg(yaml_file_path)
        .status()
        .expect("Failed to execute command");

    // Check if the command was successful
    if !status.success() {
        eprintln!("Command failed with status: {}", status);
        std::process::exit(1);
    }
}
