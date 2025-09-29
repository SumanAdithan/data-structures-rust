fn main() {
    let mut arr = vec![1, 2, 3];
    arr.push(4); // add 4 at end
    arr.insert(0, 0); // add 0 at beginning (like unshift)
    arr.pop(); // remove item from end
    arr.remove(0); // remove item from beginning

    for item in &arr {
        println!("{}", item);
    }

    // for each
    arr.iter().for_each(|n| {
        println!("{}", n);
    });

    // map
    let new_arr: Vec<_> = arr
        .iter()
        .map(|x| x)
        .collect();
    println!("{:?}", new_arr);

    // filter
    let filtered: Vec<_> = arr
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("{:?}", filtered);

    // Reduce (fold in Rust)
    let sum: i32 = arr.iter().fold(0, |acc, &x| acc + x);
    println!("{}", sum);

    // Concat
    let arr2 = vec![5, 6];
    let combined: Vec<_> = [arr.clone(), arr2].concat();
    println!("{:?}", combined);

    // Slice
    let slice = &arr[1..3]; // returns &[i32]
    println!("{:?}", slice);

    // Splice
    arr.splice(1..3, vec![7, 8]);
    println!("{:?}", arr);
}
