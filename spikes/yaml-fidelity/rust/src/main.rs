use std::{env, fs, process};
use yaml_rt::YamlDoc;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("usage: yaml-fidelity-rust WORKFLOW".into());
    }

    let input = fs::read_to_string(&args[1])?;
    let mut doc = YamlDoc::parse(&input)?;
    doc.set_scalar(&["env", "RUNTIME"], "22")?;
    print!("{}", doc);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}
