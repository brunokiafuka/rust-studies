use std::collections::HashMap;

fn main() {
    let _my_map: () = HashMap::from([("key0", 1), ("key1", 2), ("key2", 3)])
        .iter()
        .for_each(|(k, v)| println!("{}, {}", k, v));
}

// let foo: Vec<_> = vec![1, 2, 3, 4].iter().map(|x| x + 1).collect();
/*   let binding = vec![1, 2, 3, 4];
let mut foo = binding.into_iter().map(|x| x + 1);

let mut new_vec = vec![];

while let Some(x) = foo.next() {
    new_vec.push(x);
}

print!("{:?}", new_vec); */
