use hashbrown::HashMap;
use serde_scan::scan;

type Data = HashMap<Vec<String>, FsEntry>;

#[derive(Default, Debug)]
pub struct FsEntry {
    dirs: Vec<String>,
    files: Vec<File>,
    size: usize,
}

#[derive(Default, Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
}

pub fn parse(input: &str) -> Data {
    let mut fs: HashMap<Vec<String>, FsEntry> = HashMap::new();
    let mut cwd: Vec<String> = vec![];
    for line in input.lines() {
        match line {
            _ if line.starts_with("$ cd") => {
                let dir_name: &str = scan!("$ cd {}" <- line).unwrap();
                match dir_name {
                    "/" => cwd = vec!["/".into()],
                    ".." => {
                        cwd.pop();
                    }
                    dir => cwd.push(dir.into()),
                }
            }
            _ if line.starts_with("$ ls") => {}
            _ if line.starts_with("dir") => {
                fs.entry(cwd.clone())
                    .or_default()
                    .dirs
                    .push(line.replace("dir ", ""));
            }
            line => {
                let data = line.split(' ').collect::<Vec<_>>();
                fs.entry(cwd.clone()).or_default().files.push(File {
                    name: data[1].into(),
                    size: data[0].parse().unwrap(),
                });
            }
        }
    }

    let keys: Vec<_> = fs.keys().cloned().collect();
    for key in keys {
        let size = dir_size(&fs, &key);
        fs.entry(key).and_modify(|entry| entry.size = size);
    }

    // print_dir(&fs, vec!["/".into()], 0);

    fs
}

pub fn part_1(fs: &Data) -> usize {
    fs.iter()
        .map(|(_k, entry)| entry.size)
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn part_2(fs: &Data) -> usize {
    let total_used = fs.get(&vec!["/".into()]).unwrap().size;
    let unused = 70_000_000 - total_used;
    let space_needed = 30_000_000;
    fs.values()
        .map(|v| v.size)
        .filter(|size| unused + size >= space_needed)
        .min()
        .unwrap()
}

fn dir_size(fs: &HashMap<Vec<String>, FsEntry>, entry: &Vec<String>) -> usize {
    let entry_data = fs.get(entry).unwrap();
    let mut total = 0;
    for dir in &entry_data.dirs {
        let mut dir_entry = entry.clone();
        dir_entry.push(dir.clone());
        total += dir_size(fs, &dir_entry);
    }
    total += entry_data.files.iter().fold(0, |acc, f| acc + f.size);
    total
}

#[allow(unused)]
fn print_dir(fs: &HashMap<Vec<String>, FsEntry>, entry: Vec<String>, level: usize) {
    let entry_data = fs.get(&entry).unwrap();
    println!("- {} (dir) {}", entry.last().unwrap(), entry_data.size);
    for dir in entry_data.dirs.clone() {
        print_indent(level + 1);
        let mut dir_entry = entry.clone();
        dir_entry.push(dir);
        print_dir(fs, dir_entry, level + 1);
    }
    for file in entry_data.files.clone() {
        print_indent(level + 1);
        println!("- {} (file, size={})", file.name, file.size);
    }
}

#[allow(unused)]
fn print_indent(size: usize) {
    for _ in 0..size * 2 {
        print!(" ");
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 24933642);
    }
}
