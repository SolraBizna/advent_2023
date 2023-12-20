use std::collections::HashMap;

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let modules: HashMap<String, (String, Vec<String>)> = lines
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let (name, module): (&str, &str) =
                if let Some(name) = name.strip_prefix('%') {
                    (name, "FF")
                } else if let Some(name) = name.strip_prefix('&') {
                    (name, "NAND")
                } else {
                    assert_eq!(name, "broadcaster");
                    (name, "")
                };
            (
                name.to_string(),
                (
                    module.to_string(),
                    destinations.split(", ").map(|x| x.to_string()).collect(),
                ),
            )
        })
        .collect();
    for (name, (module, _destinations)) in modules.iter() {
        println!("[{name}]");
        if !module.is_empty() {
            println!("{module}")
        }
    }
    modules.iter().for_each(|(name, (_, destinations))| {
        destinations.iter().for_each(|dest| {
            println!("{name} 1--* {dest}");
        })
    });
}
