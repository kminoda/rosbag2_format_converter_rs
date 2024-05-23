# rosbag2_format_converter_rs

## Installation

```bash
cargo install --git https://github.com/kminoda/rosbag2_format_converter_rs.git
```

## Usage

```bash
# mcap -> sqlite3
rosbag_format_converter mcap-to-sqlite3 <INPUT_BAG> <OUTPUT_BAG>

# sqlite3 -> mcap
rosbag_format_converter sqlite3-to-mcap <INPUT_BAG> <OUTPUT_BAG>
```
