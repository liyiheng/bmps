use std::path::PathBuf;

use bmps::config::Size;
use bmps::Config;
use clap::{arg, ArgMatches};
fn get<T: Clone + Send + Sync + 'static>(m: &ArgMatches, id: &str) -> T {
    let msg = format!("Invalid {id}");
    m.get_one::<T>(id).expect(msg.as_str()).clone()
}
fn is_dir<P: AsRef<std::path::Path>>(path: P) -> bool {
    std::fs::metadata(path).unwrap().is_dir()
}
fn main() {
    env_logger::init();
    let matches = clap::Command::new("normalize")
        .arg(
            arg!(-w --width [WIDTH] "Width")
                .default_value("1920")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(-H --height [HEIGHT] "Height")
                .default_value("1080")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(-b --blur [BLUR] "Blur radius")
                .default_value("50")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(-r --round [ROUND] "Round radius")
                .default_value("45")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(-s --shadow [SHADOW] "Shadow width")
                .default_value("40")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(-p --padding [PADDING] "Range [0, 0.5)")
                .default_value("0.1")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(arg!(-i --input [INPUT_PATH] "File or directory path").default_value("."))
        .arg(arg!(-o --out [OUTPUT] "Output path").default_value("."))
        .arg(arg!(-W --white  "White background").value_parser(clap::value_parser!(bool)))
        .get_matches();
    let cfg = Config {
        font: None,
        size: Size {
            width: get(&matches, "width"),
            height: get(&matches, "height"),
            blur_radius: get(&matches, "blur"),
            round_radius: get(&matches, "round"),
            padding: get(&matches, "padding"),
            shadow: get(&matches, "shadow"),
        },
        source_file: get(&matches, "input"),
        dest_file: get(&matches, "out"),
        white_bg: get(&matches, "white"),
    };
    if is_dir(cfg.source_file.as_str()) {
        batch(&cfg);
    } else {
        single(&cfg);
    }
}

fn single(cfg: &Config) {
    let mut output = PathBuf::new();
    output.push(cfg.dest_file.as_str());
    if !output.exists() && output.extension().is_none() {
        std::fs::create_dir_all(&output).unwrap();
    }
    if output.extension().is_none() || output.is_dir() {
        // 若输出路径是目录，根据原文件名生成新文件名
        let p = std::path::Path::new(cfg.source_file.as_str());
        let stem = p.file_stem().unwrap();
        let mut s = stem.to_os_string();
        s.push("_bmps.jpg"); // FIXME: 若文件已存在，加编号
        output.push(s);
    }
    let mut c = cfg.clone();
    c.dest_file = output.to_str().unwrap().to_owned();
    bmps::go(c).unwrap();
}
fn batch(cfg: &Config) {
    let mut output = PathBuf::from(cfg.dest_file.as_str());
    if cfg.source_file == cfg.dest_file {
        output.push("bmps");
    }
    if output.exists() && !output.is_dir() {
        panic!("Invalid output");
    }
    if !output.exists() {
        std::fs::create_dir_all(output.as_path()).unwrap();
    }
    let entries: Vec<_> = std::fs::read_dir(cfg.source_file.as_str())
        .unwrap()
        .filter_map(|r| match r {
            Err(e) => {
                log::warn!("{e:?}");
                None
            }
            Ok(d) => Some(d),
        })
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().is_some())
        .collect();
    for f in entries {
        output.push(f.file_name());
        let mut c = cfg.clone();
        c.source_file = f.path().to_string_lossy().to_string();
        c.dest_file = output.to_string_lossy().to_string();
        output.pop();
        if let Err(e) = bmps::go(c) {
            log::warn!("{:?} {e:?}", f.file_name());
        }
    }
}
