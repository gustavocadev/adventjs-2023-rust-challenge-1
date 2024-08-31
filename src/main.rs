#[derive(Debug)]
struct Id {
  value: i32,
  idx: i32,
}

fn find_first_repeated(gifs: Vec<i32>) -> i32 {
  let gifs_cloned = gifs.clone();

  let mut repeated_ids: Vec<Id> = vec![];

  for (idx, el) in gifs.into_iter().enumerate() {
    let index_value_found = gifs_cloned[idx + 1..].iter().position(|&x| x == el);

    if let Some(idx) = index_value_found {
      let repeated_value = gifs_cloned[idx + 1];

      repeated_ids.push(Id {
        idx: idx as i32,
        value: repeated_value,
      })
    }
  }

  repeated_ids.sort_by(|a, b| a.idx.cmp(&b.idx));

  if let Some(el) = repeated_ids.get(0) {
    return el.value;
  }
  return -1;
}

fn main() {
  let gift_ids = vec![5, 1, 5, 1];

  println!(
    "El numero que se repite es {}",
    find_first_repeated(gift_ids)
  );
}
