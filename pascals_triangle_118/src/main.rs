#[cfg(test)]
mod test;

fn main() {
  generate(5);
}

#[allow(dead_code)]
fn generate(num_rows: u32) -> Vec<Vec<u32>>
{ if num_rows == 0 { return vec![]; }
  if num_rows == 1 { return vec![vec![1]]; }

  let mut triangle = vec![vec![1], vec![1, 1]];

  for _ in 2..num_rows
  { let mut new_row: Vec<u32> = vec![];

    new_row.push(1);

    triangle.last().unwrap().iter()
      .zip(triangle.last().unwrap().iter().skip(1))
      .for_each(|(n1, n2)| new_row.push(n1 + n2));

    new_row.push(1);

    triangle.push(new_row);
  }

  return triangle;
}
