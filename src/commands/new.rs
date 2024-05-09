use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

pub fn execute(contest_number: &str) {
    let project_dir = Path::new(contest_number);

    if project_dir.exists() {
        println!("Directory already exists: {}", project_dir.display());
    } else {
        if let Err(err) = create_cargo_project(project_dir) {
            eprintln!("Failed to create Cargo project: {}", err);
        }

        if let Err(err) = create_src_files(project_dir) {
            eprintln!("Failed to create source files: {}", err);
        }

        if let Err(err) = set_dependency(project_dir) {
            eprintln!("Failed to set dependency: {}", err);
        }
    }
}

fn create_cargo_project(project_dir: &Path) -> std::io::Result<()> {
    Command::new("cargo").arg("new").arg(project_dir).status()?;
    Ok(())
}

fn create_src_files(project_dir: &Path) -> std::io::Result<()> {
    let src_dir = project_dir.join("src");

    fs::remove_file(src_dir.join("main.rs"))?;

    for c in b'a'..b'h' {
        let filename = format!("{}.rs", c as char);
        let filepath = src_dir.join(filename);

        let mut file = File::create(filepath)?;
        file.write_all(b"fn main() {\n    println!(\"Hello, world!\");\n}\n")?;
    }

    Ok(())
}

fn set_dependency(project_dir: &Path) -> std::io::Result<()> {
    let toml_path = project_dir.join("Cargo.toml");
    let dependency = r#"proconio = "0.4.3""#;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(toml_path.clone())?;
    let reader = BufReader::new(&file);
    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let dependencies_index = lines
        .iter()
        .position(|line| line.contains("[dependencies]"));

    if let Some(index) = dependencies_index {
        lines.insert(index + 1, dependency.to_string());
        let mut writer = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(toml_path)?;
        for line in &lines {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}
