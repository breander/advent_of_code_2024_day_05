use std::env;
use std::fs;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();

    let mut rules: Vec<&str> = Vec::new();
    let mut updates: Vec<&str> = Vec::new();

    for line in lines {
        if line.contains("|") {
            rules.push(line);
        }

        if line.contains(",") {
            updates.push(line);
        }
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for update in &updates {
        let mut pages = update.split(",").collect::<Vec<&str>>();
        let (mut valid, mut index1, mut index2) = check_if_update_valid(&pages, &rules);
        let length = pages.len();

        if valid {
            let middle = pages[length/2].parse::<u32>().unwrap();
            println!("✅ {update}");
            part1_sum += middle;
        }
        else {
            println!("❌ {update}");

            loop {
                pages.swap(index1, index2);
                (valid, index1, index2) = check_if_update_valid(&pages, &rules);

                if valid {
                    println!("✅ {}", pages.join(","));
                    break;
                }
                println!("❌ {}", pages.join(","));
            }

            let middle = pages[length/2].parse::<u32>().unwrap();
            part2_sum += middle;
        }
    }

    println!("part 1 sum: {part1_sum}");
    println!("part 2 sum: {part2_sum}");
}

fn check_if_update_valid<'a>(pages: &'a Vec<&str>, rules: &'a Vec<&str>) -> (bool,usize,usize) {
    for (index1, page) in pages.iter().enumerate() {
        let matched_rules = rules.iter().filter(|rule| &rule.split("|").collect::<Vec<&str>>()[0] == page); 
        for rule in matched_rules {
            let second_page = rule.split("|").collect::<Vec<&str>>()[1];
            if let Some(index2) = pages.iter().position(|p| *p == second_page) {
                if index2 < index1 {
                    return (false, index1, index2);
                }
            }
        }
    }
    return (true, 0, 0);
}
