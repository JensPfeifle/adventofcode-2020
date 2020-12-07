use std::collections::HashSet;

fn part1(groups: &Vec<String>) -> usize {
    let mut set = HashSet::new();
    for group in groups {
        set.clear();
        println!("{:?}", group);
        set.insert(group);
    }
    0
}

fn main() -> std::io::Result<()> {
    const example: &str = concat!(
        "abc\n", "\n", "a\n", "b\n", "c\n", "\n", "ab\n", "ac\n", "\n", "a\n", "a\n", "a\n", "a\n",
        "\n", "b\n",
    );
    let input = include_str!("../6.in").trim();
    let mut groups = Vec::<String>::new();
    for group in example.split("\n\n") {
        for line in group.lines() {
            groups.push(line.to_string());
        }
    }
    let p1 = part1(&groups);

    Ok(())
}
