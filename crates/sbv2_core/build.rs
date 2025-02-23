use std::env;
use std::fs;
use std::io::copy;
use std::path::PathBuf;

use home_dir::HomeDirExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let static_path = "~/.cache/sbv2/all.bin".expand_home().unwrap();
    let out_path = PathBuf::from(&env::var("OUT_DIR").unwrap()).join("all.bin");
    println!("cargo:rerun-if-changed=build.rs");
    if static_path.exists() {
        fs::hard_link(static_path, out_path).unwrap();
    } else {
        println!("cargo:warning=Downloading dictionary file...");
        let mut response =
            ureq::get("https://huggingface.co/neody/sbv2-api-assets/resolve/main/dic/all.bin")
                .call()?;
        let mut response = response.body_mut().as_reader();
        let mut file = fs::File::create(&out_path)?;
        copy(&mut response, &mut file)?;
    }
    Ok(())
}
