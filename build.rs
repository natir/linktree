/// Download static dependency
/* std use */
use std::io::Write;

/* crates use */

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

fn fetch_extract<P>(
    uri: &str,
    target2dest: std::collections::HashMap<String, P>,
) -> std::result::Result<(), Error>
where
    P: std::convert::AsRef<std::path::Path> + std::fmt::Display,
{
    let response = reqwest::blocking::get(uri)?.bytes()?;
    let mut arxiv = tar::Archive::new(flate2::read::MultiGzDecoder::new(&*response));

    let mut entries = arxiv.entries()?;
    while let Some(Ok(mut entry)) = entries.next() {
        if let Some(dest) = target2dest.get(
            &entry
                .path()?
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        ) {
            println!("cargo:rerun-if-changed=\"{}\"", dest);
            entry.unpack(dest)?;
        }
    }

    Ok(())
}

fn fetch_file<P>(path: P, uri: &str) -> std::result::Result<(), Error>
where
    P: std::convert::AsRef<std::path::Path> + std::fmt::Display,
{
    let mut file = std::fs::File::create(&path)?;
    let content = reqwest::blocking::get(uri)?;
    file.write_all(&content.bytes()?)?;
    file.flush()?;

    println!("cargo:rerun-if-changed=\"{}\"", path);

    Ok(())
}

pub fn main() -> std::result::Result<(), Error> {
    println!("cargo:rerun-if-changed=\"build.rs\"");

    std::fs::create_dir_all("static/styles/forkawesome/")?;
    std::fs::create_dir_all("static/fonts/forkawesome/")?;

    fetch_extract(
        "https://github.com/ForkAwesome/Fork-Awesome/archive/refs/tags/1.2.0.tar.gz",
        [
            (
                "_animated.scss".to_string(),
                "static/styles/forkawesome/_animated.scss".to_string(),
            ),
            (
                "_bordered-pulled.scss".to_string(),
                "static/styles/forkawesome/_bordered-pulled.scss".to_string(),
            ),
            (
                "_core.scss".to_string(),
                "static/styles/forkawesome/_core.scss".to_string(),
            ),
            (
                "_fixed-width.scss".to_string(),
                "static/styles/forkawesome/_fixed-width.scss".to_string(),
            ),
            (
                "_functions.scss".to_string(),
                "static/styles/forkawesome/_functions.scss".to_string(),
            ),
            (
                "_icons.scss".to_string(),
                "static/styles/forkawesome/_icons.scss".to_string(),
            ),
            (
                "_larger.scss".to_string(),
                "static/styles/forkawesome/_larger.scss".to_string(),
            ),
            (
                "_list.scss".to_string(),
                "static/styles/forkawesome/_list.scss".to_string(),
            ),
            (
                "_mixins.scss".to_string(),
                "static/styles/forkawesome/_mixins.scss".to_string(),
            ),
            (
                "_path.scss".to_string(),
                "static/styles/forkawesome/_path.scss".to_string(),
            ),
            (
                "_rotated-flipped.scss".to_string(),
                "static/styles/forkawesome/_rotated-flipped.scss".to_string(),
            ),
            (
                "_screen-reader.scss".to_string(),
                "static/styles/forkawesome/_screen-reader.scss".to_string(),
            ),
            (
                "_stacked.scss".to_string(),
                "static/styles/forkawesome/_stacked.scss".to_string(),
            ),
            (
                "_variables.scss".to_string(),
                "static/styles/forkawesome/_variables.scss".to_string(),
            ),
            (
                "forkawesome-webfont.eot".to_string(),
                "static/fonts/forkawesome/webfont.eot".to_string(),
            ),
            (
                "forkawesome-webfont.svg".to_string(),
                "static/fonts/forkawesome/webfont.svg".to_string(),
            ),
            (
                "forkawesome-webfont.ttf".to_string(),
                "static/fonts/forkawesome/webfont.ttf".to_string(),
            ),
            (
                "forkawesome-webfont.woff".to_string(),
                "static/fonts/forkawesome/webfont.woff".to_string(),
            ),
            (
                "forkawesome-webfont.woff2".to_string(),
                "static/fonts/forkawesome/webfont.woff2".to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<String, String>>(),
    )?;

    std::fs::create_dir_all("static/styles/academicons/")?;
    std::fs::create_dir_all("static/fonts/academicons/")?;

    fetch_extract(
        "https://github.com/jpswalsh/academicons/archive/refs/tags/v1.9.2.tar.gz",
        [
            (
                "academicons.min.css".to_string(),
                "static/styles/academicons/upstream.css".to_string(),
            ),
            (
                "academicons.eot".to_string(),
                "static/fonts/academicons/webfont.eot".to_string(),
            ),
            (
                "academicons.svg".to_string(),
                "static/fonts/academicons/webfont.svg".to_string(),
            ),
            (
                "academicons.ttf".to_string(),
                "static/fonts/academicons/webfont.ttf".to_string(),
            ),
            (
                "academicons.woff".to_string(),
                "static/fonts/academicons/webfont.woff".to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<String, String>>(),
    )?;

    Ok(())
}
