fn main() {
    let x = plus_one(2);

    println!("We added a number: {:?}", x);

    let x = plus_two(x);

    println!("We added 2 to a number: {:?}", x);
}

fn plus_one(x: i32) -> i32 {
  x+1
}

fn plus_two(x: i32) -> i32 {
  return x+2;
}