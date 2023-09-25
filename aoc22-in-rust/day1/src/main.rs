const INPUT: &'static str = r#"
        1000
        2000
        3000
        
        4000

        5000
        6000

        7000
        8000
        9000

        10000
"#;

fn main() {
    let lines: Vec<String> = INPUT
        .trim()
        .lines()
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect();

    let list_of_lists: Vec<Vec<String>> = lines
        .into_iter()
        .fold(Vec::new(), |mut acc, s| {
            if s.is_empty() {
                acc.push(Vec::new());
            } else {
                if acc.is_empty() || acc.last().unwrap().is_empty() {
                    acc.push(vec![s]);
                } else {
                    acc.last_mut().unwrap().push(s);
                }
            }
            acc
        })
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect();

    let mut list_of_strings_to_ints: Vec<i32> = list_of_lists
        .into_iter()
        .map(|list| {
            list.into_iter()
                .filter_map(|x| x.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect();

    let list_of_strings_to_ints_ref = &list_of_strings_to_ints;
    let maximum_calories: i32 = *list_of_strings_to_ints_ref.iter().max().unwrap();

    list_of_strings_to_ints.sort();
    list_of_strings_to_ints.reverse();
    let top_3_elves: &i32 = &list_of_strings_to_ints[0..3].into_iter().sum();

    println!("{:?}", top_3_elves);
}
