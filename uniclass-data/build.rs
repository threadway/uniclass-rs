use serde::Deserialize;
use std::{collections::HashMap, io::Write, path::Path};
use uniclass::Uniclass;

fn main() {
    // Find data files
    let mut data_paths = Vec::new();
    for entry in std::fs::read_dir("uniclass_tables").unwrap() {
        let entry = entry.unwrap();
        if entry
            .path()
            .extension()
            .map(|ext| ext == "csv")
            .unwrap_or(false)
        {
            data_paths.push(entry.path());
        }
    }

    // Tell Cargo that if the given file changes, to rerun this build script.
    let mut uniclass_map = HashMap::new();
    for data_path in data_paths.iter() {
        println!("cargo:rerun-if-changed={:?}", data_path);
        decode_from_csv(&mut uniclass_map, data_path);
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut f = std::fs::File::create(dest_path).unwrap();
    f.write_all(
        b"
use uniclass::{Uniclass,UniclassTable};
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref UNICLASS_TITLES: HashMap<Uniclass, &'static str> = {
        let mut m = HashMap::new();
",
    )
    .unwrap();
    for (k, v) in uniclass_map.into_iter() {
        writeln!(
            f,
            "        m.insert(Uniclass::new(UniclassTable::{},{},{:?},{:?},{:?}), \"{}\");",
            k.table(),
            k.group(),
            k.sub_group(),
            k.section(),
            k.object(),
            v
        )
        .unwrap();
    }
    f.write_all(
        b"
        m
    };
}
",
    )
    .unwrap();
    // println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());
    println!("cargo:rustc-cfg=has_generated_feature");
}

#[derive(Debug, Deserialize)]
struct UniclassRecord {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Title")]
    title: String,
    // #[serde(rename = "NBS Code")]
    // nbs_code: Option<String>,
    // #[serde(rename = "NRM")]
    // nrm: Option<String>,
}

fn decode_from_csv(uniclass_map: &mut HashMap<Uniclass, String>, path: &Path) {
    let f = std::fs::File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: UniclassRecord = result.unwrap();
        eprintln!("{}", record.code);
        let item: Uniclass = record.code.parse().unwrap();
        println!("{:?} \"{}\"", item, record.title);
        uniclass_map.insert(item, record.title);
    }
}
