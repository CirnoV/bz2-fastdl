use bzip2::{read::BzEncoder, Compression};
use rayon::prelude::*;
use std::{
    fs::File,
    io,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

#[derive(StructOpt, Debug)]
#[structopt(name = "bz2-fastdl")]
struct Opt {
    #[structopt(name = "PATH", parse(from_os_str))]
    root: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

fn main() {
    let fastdl_ext = [
        "vmt", "vtf", "vtx", "phy", "mdl", "vvd", "wav", "mp3", "bsp",
    ];
    let opt = Opt::from_args();
    let walker = WalkDir::new(opt.root).follow_links(true).into_iter();
    let files = walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| {
            let entry = match e {
                Ok(e) => e,
                Err(_) => return None,
            };
            if !is_file(&entry) {
                return None;
            }
            let path = entry.into_path();
            let extension = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let filename = path.file_name().unwrap().to_str().unwrap();

            if !fastdl_ext.contains(&extension) {
                return None;
            }
            if path.with_file_name(format!("{}.bz2", filename)).exists() {
                return None;
            }
            Some(path)
        })
        .collect::<Vec<_>>();

    let count = AtomicUsize::new(1);
    let file_num: usize = files.len();
    files.into_par_iter().for_each(|path| {
        let f = File::open(&path).unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();

        let mut bzip_encoder = BzEncoder::new(f, Compression::best());
        let mut w = File::create(path.with_file_name(format!("{}.bz2", filename))).unwrap();
        io::copy(&mut bzip_encoder, &mut w).unwrap();

        println!(
            "[{}/{}] {}",
            count.fetch_add(1, Ordering::SeqCst),
            file_num,
            path.display()
        );
    });
}
