use std::collections::HashMap;

fn main() {
  // create
  let mut scores = HashMap::new();

  // add entity
  scores.insert(String::from("Team A"), 10);
  scores.insert(String::from("Team B"), 20);
  println!("{:?}", scores);

  // find by key
  if let Some(score) = scores.get("Team A") {
    println!("Team A have score: {}", score);  
  } else {
    println!("Team A have no score");
  } 

  // change value by key
  scores.insert(String::from("Team A"), 30);
  println!("{:?}", scores);

  // iterate 
  for (key, value) in &scores {
    println!("{} | {}", key, value);
  }

  //add and remove
  scores.insert(String::from("Team C"), 30);
  scores.remove("Team C");
  
  println!("{:?}", scores);
  
}