use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 4, 3, 5, 2, 4, 5];
    v.sort();

    let length = v.len() - 1;
    let middle = length / 2;

    match v.get(middle) {
        Some(value) => println!("The median is {}", value),
        None => println!("Nothing here."),
    }

    let mut map = HashMap::new();
    for element in v {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, 0);
    for (key, value) in map {
        if value > mode.1 {
            mode = (key, value)
        }
    }
    println!("The mode is {}", mode.0)
}
