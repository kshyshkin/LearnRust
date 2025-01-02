fn main() {
  // let mut v: Vec<i32> = Vec::new();
  // v.push(10);
  // v.push(20);
  // v.push(30);

  let v = vec![10,20,30];

  println!("first elem {}", v[0]);

  match v.get(3) {
    Some(value) => println!("second elem {}", value),
    None => println!("Non elem found"),
  }

  let mut books_v: Vec<String> = Vec::new();

  books_v.push("Help".to_string());
  books_v.push("Me".to_string());

  println!("first book: {}", books_v[0]);

  match books_v.get(1) {
    Some(value) => println!("second: {}", value),
    None => println!("To DO"),
  }

  for value in &mut books_v {
    *value += "_changed";
  }

  books_v.pop();
  books_v.pop();
  books_v.remove(0);
  
  println!("{:?}", books_v);
}