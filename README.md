# bz2-fastdl

This tool compresses files commonly used for game server fast download (FastDL) using bzip2 compression. It processes specified input directories, identifies files with relevant extensions, and outputs compressed versions (`.bz2`) to a designated output directory, preserving the relative directory structure.

## Installation

Assuming you have Rust and Cargo installed, you can install `bz2-fastdl` directly from crates.io:

```bash
cargo install bz2-fastdl
```

## Usage

```bash
bz2-fastdl -i <input_dir1> [input_dir2 ...] -o <output_root>
```

### Arguments

*   `-i, --input-dir <INPUT_DIR>`: One or more input directories to scan for files. The tool will recursively search these directories.
*   `-o, --output-root <OUTPUT_ROOT>`: The root directory where the compressed files will be saved. The tool will create subdirectories within this root directory based on the base name of each input directory and the relative path of the original file.

### Example

Suppose you have the following directory structure:

```
/path/to/game/materials/
├── file1.vmt
└── textures/
    └── texture1.vtf

/path/to/game/models/
└── player.mdl
```

To compress the files in `materials` and `models` directories and save them to `/path/to/fastdl`, you would run:

```bash
bz2-fastdl -i /path/to/game/materials /path/to/game/models -o /path/to/fastdl
```

This will create the following structure in `/path/to/fastdl`:

```
/path/to/fastdl/
├── materials/
│   ├── file1.vmt.bz2
│   └── textures/
│       └── texture1.vtf.bz2
└── models/
    └── player.mdl.bz2
```

### Supported File Extensions

The tool currently compresses files with the following extensions:

*   `vmt`
*   `vtf`
*   `vtx`
*   `phy`
*   `mdl`
*   `vvd`
*   `wav`
*   `mp3`
*   `bsp`
*   `nav`

Files with other extensions will be ignored. Hidden files and directories (those starting with a `.`) are also skipped.
