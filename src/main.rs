use anyhow::Result;
use bzip2::{read::BzEncoder, Compression};
use clap::Parser;
use rayon::prelude::*;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    io::{BufReader, BufWriter},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "bz2-fastdl")]
struct Args {
    #[arg(short = 'i', required = true, num_args = 1..)]
    input_dir: Vec<PathBuf>,

    #[arg(short = 'o')]
    output_root: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

struct FileToProcess {
    input_path: PathBuf,
    base_path: PathBuf,
}

fn main() -> Result<()> {
    let fastdl_ext = [
        "vmt", "vtf", "vtx", "phy", "mdl", "vvd", "wav", "mp3", "bsp", "nav",
    ];
    let args = Args::parse();

    fs::create_dir_all(&args.output_root)?;

    let files_to_process = args
        .input_dir
        .iter()
        .filter(|dir| dir.is_dir())
        .flat_map(|input_dir| {
            WalkDir::new(input_dir)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
                .filter_map(move |e| {
                    let Ok(entry) = e else {
                        return None;
                    };

                    let Ok(metadata) = entry.metadata() else {
                        return None;
                    };

                    if !metadata.is_file() {
                        return None;
                    }

                    let path = entry.into_path();
                    let extension = path.extension()?.to_str()?;

                    if !fastdl_ext.contains(&extension) {
                        return None;
                    }

                    Some(FileToProcess {
                        input_path: path,
                        base_path: input_dir.clone(),
                    })
                })
        })
        .collect::<Vec<_>>();

    let count = AtomicUsize::new(1);
    let file_num: usize = files_to_process.len();

    files_to_process.into_par_iter().for_each(|file_info| {
        let input_path = file_info.input_path;
        let base_path = file_info.base_path;

        let Ok(relative_path) = input_path.strip_prefix(&base_path) else {
            return;
        };

        let Some(base_name) = base_path.file_name() else {
            return;
        };

        let mut output_path = args.output_root.join(base_name).join(relative_path);

        let Some(original_filename) = output_path.file_name().map(|s| s.to_os_string()) else {
            return;
        };

        let mut bz2_filename = original_filename;
        bz2_filename.push(".bz2");
        output_path.set_file_name(bz2_filename);

        if output_path.exists() {
            return;
        }

        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("경고: 디렉토리 생성 실패 '{}': {}", parent.display(), e);
                    return;
                }
            }
        }

        let Ok(mut bzip_encoder) = File::open(&input_path)
            .map(BufReader::new)
            .map(|r| BzEncoder::new(r, Compression::fast()))
        else {
            return;
        };

        let Ok(mut writer) = File::create(&output_path).map(BufWriter::new) else {
            return;
        };

        if io::copy(&mut bzip_encoder, &mut writer).is_ok() {
            println!(
                "[{}/{}] {} -> {}",
                count.fetch_add(1, Ordering::SeqCst),
                file_num,
                input_path.display(),
                output_path.display()
            );
        }
    });

    Ok(())
}
