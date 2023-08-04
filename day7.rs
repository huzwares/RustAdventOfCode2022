use std::collections::HashMap;
use std::io;

enum FileSys {
    File(usize),
    Dir(HashMap<String, FileSys>),
}

impl FileSys {
    fn file_size(&self) -> usize {
        match self {
            FileSys::File(size) => *size,
            FileSys::Dir(dir) => dir.values().map(|f| f.file_size()).sum(),
        }
    }

    fn is_directory(&self) -> bool {
        matches!(self, FileSys::Dir(_))
    }

    fn visit(&self, f: &mut dyn FnMut(&FileSys)) {
        f(self);
        if let FileSys::Dir(dir) = self {
            dir.values().for_each(|d| d.visit(f));
        }
    }
}

fn navigate(
    directory: &'_ mut HashMap<String, FileSys>,
    location: impl IntoIterator<Item = impl AsRef<str>>,
) -> &'_ mut HashMap<String, FileSys> {
    let mut current = directory;
    for name in location {
        current = match current.get_mut(name.as_ref()) {
            Some(FileSys::Dir(dir)) => dir,
            _ => {
                eprintln!("Directory not found");
                std::process::exit(1);
            }
        }
    }
    current
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut root: HashMap<String, FileSys> = HashMap::new();
    let mut cursor: Vec<String> = Vec::new();
    let mut current_dir = &mut root;
    for line in input.lines().skip(1) {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "$" => {
                if split[1] == "cd" {
                    if split[2] == ".." {
                        cursor.pop();
                    } else {
                        cursor.push(split[2].to_owned());
                    }
                    current_dir = navigate(&mut root, &cursor);
                }
            }
            "dir" => {
                current_dir.insert(split[1].to_owned(), FileSys::Dir(Default::default()));
            }
            size => {
                current_dir.insert(
                    split[1].to_owned(),
                    FileSys::File(size.parse::<usize>().unwrap()),
                );
            }
        }
    }
    let root = FileSys::Dir(root);

    // Part 1
    let mut sum = 0;
    root.visit(&mut |entry| {
        let size = entry.file_size();
        if entry.is_directory() && size <= 100000 {
            sum += entry.file_size();
        }
    });
    println!("Part 1: {}", sum);

    // Part 2
    let mut min = usize::MAX;
    let unused = 70000000 - root.file_size();
    let space_needed = 30000000 - unused;
    root.visit(&mut |entry| {
        let size = entry.file_size();
        if size >= space_needed && size < min {
            min = size;
        }
    });
    println!("Part 2: {}", min);
}