use clap::Parser;
use sha::{
    sha1::Sha1,
    utils::{Digest, DigestExt},
};
use std::{fs::File, io::Read};

#[derive(Debug, Parser)]
#[command(author = "selune", version, about, long_about = None)]
struct Args {
    // 文件路径
    #[arg(short, long, value_name = "file path", default_value = "Cargo.toml")]
    file: String,

    // 加密类型
    #[arg(
        short = 't',
        long = "type",
        value_name = "crypto type",
        help = "eg: md5",
        default_value = "md5"
    )]
    crypto_type: String,
}

fn main() {
    let args = Args::parse();
    let path = args.file;
    let crypto_type = args.crypto_type;

    // 读取文件
    let mut contents = String::new();
    File::open(&path)
        .expect(format!("Unable to open file: {}", &path).as_str())
        .read_to_string(&mut contents)
        .unwrap();

    match crypto_type.as_str() {
        "md5" => {
            let md5_digest = md5::compute(contents);
            println!("MD5 ({}) = {:?}", &path, &md5_digest);
        }
        "sha1" => {
            let sha1_digest = Sha1::default().digest(contents.as_bytes()).to_hex();
            println!("SHA1 ({}) = {:?}", &path, &sha1_digest);
        }
        _ => println!("crypto type is null"),
    }
}
