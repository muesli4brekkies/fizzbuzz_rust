fn main() {
  println!(
    "{}",
    (0..100).fold(String::new(), |a, n| {
      [
        a,
        (|n: i32| {
          (|s: String| (!s.is_empty()).then_some(s))(
            [
              //(2, "Be"),
              (3, "Fizz"),
              //(4, "Boop"),
              (5, "Buzz"),
              //(6, "Feep"),
              //(7, "Fob"),
              //(8, "Barf"),
              //(9, "Flab"),
            ]
            .into_iter()
            .fold(String::new(), |a, (c, s)| {
              [
                a,
                match n % c {
                  0 => s.to_string(),
                  _ => String::new(),
                },
              ]
              .join("")
            }),
          )
        })(n)
        .unwrap_or(n.to_string()),
      ]
      .join("\n")
    })
  )
}
