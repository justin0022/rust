#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filter_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("sneaker") },
    Shoe { size: 4,  style: String::from("babyshoes") },
    Shoe { size: 7,  style: String::from("boots") },
  ];

  let shoes_in_my_size = shoe_in_my_size(shoes, 7);

  assert_eq!(
    shoes_in_my_size,
    vec![
      Shoe { size: 7, style: String::from("boots") }
    ]
  )
}