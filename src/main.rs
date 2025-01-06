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

  // check and add only if entry not exist
  scores.entry(String::from("Team C")).or_insert(30);
  scores.entry(String::from("Team A")).or_insert(10);

  // update value based on current 
  let score =  scores.entry(String::from("Team A")).or_insert(10);
  *score+=5;
  
  println!("{:?}", scores);

  // practical example

  let string = "Hello hello my beautiful world my";
  let mut word_count = HashMap::new();

  for word in string.split_whitespace() {
    let wc = word_count.entry(word.to_lowercase()).or_insert(0);
    *wc+=1;
  }
  println!("{:?}", word_count);
}