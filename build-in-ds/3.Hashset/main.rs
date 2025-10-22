use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = [1, 2, 3].iter().copied().collect();

    // add elements
    set.insert(4);
    set.insert(4); // duplicate ignored

    // check element exists
    println!("{}", set.contains(&4));

    // delete individual element
    set.remove(&3);

    // size
    println!("{}", set.len());

    // delete all elements in the set
    set.clear();

    // iterate set using for in
    for item in &set {
        println!("{}", item);
    }
}
