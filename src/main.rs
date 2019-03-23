#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
use std::vec::Vec;

use clap::{App, Arg};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, Clone)]
struct Package {
    name: String,
    url: Option<String>,
    github: Option<String>,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct List {
    items: BTreeMap<String, Vec<Package>>,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "### {}", self.name)?;
        write!(f, "\n")?;

        if let Some(i) = &self.url {
            write!(f, " [<img src=\"https://img.shields.io/badge/URL-homepage-navy.svg?style=for-the-badge\">]({})", i)?;
        }

        write!(f, " [<img src=\"https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=for-the-badge\">](https://crates.io/crates/{})", self.name)?;

        if let Some(i) = &self.github {
            write!(f, " [<img src=\"https://img.shields.io/badge/URL-GitHub-navy.svg?style=for-the-badge\">](https://github.com/{})", i)?;
        }

        if let Some(i) = &self.github {
            write!(f, " <img src=\"https://img.shields.io/github/last-commit/{}.svg?style=for-the-badge\">", i)?;
            write!(f, " <img src=\"https://img.shields.io/github/tag/{}.svg?style=for-the-badge\">", i)?;
//            write!(f, " <img src=\"https://img.shields.io/github/commit-activity/y/{}.svg?style=for-the-badge\">", gh)?;
        }


        write!(f, " <img src=\"https://img.shields.io/crates/d/{}.svg?style=for-the-badge\">", self.name)?;
        write!(f, " <img src=\"https://img.shields.io/crates/dv/{}.svg?style=for-the-badge\">", self.name)?;
        write!(f, " <img src=\"https://img.shields.io/crates/l/{}.svg?style=for-the-badge\">", self.name)?;


        write!(f, "\n\n")?;
        write!(f, "{}", self.description)
    }
}

fn read(path: &str) -> io::Result<List> {
    let ppath = path::PathBuf::from(path);
    let contents = fs::read_to_string(ppath)?;
    Ok(serde_yaml::from_str(&contents).unwrap())
}


fn linkize(s: &str) -> String {
    s.to_lowercase().replace(" ", "-")
}


fn write(path: &str, data: &List) -> io::Result<()> {
    let ppath = path::PathBuf::from(path);
    let mut file = File::create(ppath)?;
    file.write_all(b"## Menu\n")?;
    for (k, _) in &data.items {
        let link = linkize(k);
        file.write_all(format!("- [{}]({})\n", &k, link).as_bytes())?;
    }
    for (k, v) in &data.items {
        file.write_all(format!("##{}\n", &k).as_bytes())?;

        let mut vc = v.clone();
        vc.sort();
        for p in vc {
            file.write_all(format!("{}\n", p).as_bytes())?;
        }
    }

    Ok(())
}

//fn show(data: &List) -> io::Result<()> {
//    let s = serde_yaml::to_string(&data).unwrap();
//    println!("{}", &s);
//    Ok(())
//}

fn main() {
    let matches = App::new("Rust tools list generator")
        .version("1.0")
        .author("Paweł Kraszewski <root@linuxpedia.pl>")
        .about("Generates useful markdown of YAML crate list")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .help("Sets the input file to use")
            .default_value("items.yaml")
        )
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("Sets the output file to use")
            .default_value("README.md")
        )
        .get_matches();

    let infile = matches.value_of("INPUT").unwrap();
    let outfile = matches.value_of("OUTPUT").unwrap();

    let data = read(infile).unwrap();
    write(outfile, &data).unwrap();
}
