use std::fs::File;
use std::io::Write;
use std::process::Command;
use structopt::StructOpt;
use tempfile::NamedTempFile;

/// CLI arguments struct
#[derive(StructOpt, Debug)]
#[structopt(name = "rosbag_format_converter")]
struct Cli {
    /// Conversion type: either "mcap_to_sqlite3" or "sqlite3_to_mcap"
    #[structopt(subcommand)]
    conversion_type: ConversionType,
}

#[derive(StructOpt, Debug)]
enum ConversionType {
    McapToSqlite3(ConvertArgs),
    Sqlite3ToMcap(ConvertArgs),
}

#[derive(StructOpt, Debug)]
struct ConvertArgs {
    /// Input bag file
    input_bag: String,
    /// Output bag file
    output_bag: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    match args.conversion_type {
        ConversionType::McapToSqlite3(args) => {
            convert_bag(&args.input_bag, &args.output_bag, "sqlite3").await;
        }
        ConversionType::Sqlite3ToMcap(args) => {
            convert_bag(&args.input_bag, &args.output_bag, "mcap").await;
        }
    }
}

async fn convert_bag(input_bag: &str, output_bag: &str, storage_id: &str) {
    // Create a temporary YAML file
    let mut yaml_file = NamedTempFile::new().expect("Failed to create temporary file");

    // Write the YAML content to the temporary file
    write!(
        yaml_file,
        "output_bags:\n  - uri: {}\n    storage_id: {}\n    all: true\n",
        output_bag, storage_id
    )
    .expect("Failed to write to temporary file");

    // Get the path of the temporary file
    let yaml_file_path = yaml_file.path().to_str().expect("Invalid file path");

    // Run the conversion command
    let status = Command::new("ros2")
        .arg("bag")
        .arg("convert")
        .arg("-i")
        .arg(input_bag)
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
