use std::{fs, path::Path};

fn main() {
    let template = Path::new(file!()).parent().unwrap().join("template.rs");
    let src = Path::new(file!()).canonicalize().unwrap();
    let src = src.ancestors().nth(3).unwrap();
    let current_day = src
        .read_dir()
        .unwrap()
        .filter_map(|entry| entry.unwrap().file_name().into_string().ok())
        .filter_map(|entry| {
            entry.starts_with("day").then(|| {
                entry
                    .trim_start_matches("day")
                    .split_once('.')
                    .unwrap()
                    .0
                    .parse::<u8>()
                    .unwrap()
            })
        })
        .max()
        .unwrap();
    let new_day = current_day + 1;
    println!("Creating new day {current_day} => {new_day}");
    // Copy the template source file
    fs::copy(template, src.join(format!("day{new_day}.rs"))).unwrap();
    // Update the day range in main.rs file
    let main = src.join("main.rs");
    let mut main_content = fs::read_to_string(&main).unwrap();
    main_content = main_content.replace(
        &format!("seq!(D in 1..={current_day} "),
        &format!("seq!(D in 1..={new_day} "),
    );
    fs::write(&main, main_content).unwrap();
    // Create the input/output file
    let root = src.parent().unwrap();
    for file_path in &[
        format!("inputs/day{new_day}.txt"),
        format!("examples/inputs/day{new_day}.txt"),
        format!("examples/outputs/day{new_day}-p1.txt"),
        format!("examples/outputs/day{new_day}-p2.txt"),
    ] {
        let file_path = root.join(file_path);
        if !file_path.exists() {
            fs::write(file_path, "").unwrap();
        }
    }
}
