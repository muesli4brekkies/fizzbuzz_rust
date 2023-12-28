//fn main(){println!("{}",(0..100).fold(String::new(),|a,n|{[a,(|n:i32|{(|s:String|(!s.is_empty()).then_some(s))([(3,"Fizz"),(5,"Buzz"),].into_iter().fold(String::new(),|a,(c,s)|{[a,match n%c{0=>s.to_string(),_=>String::new(),},].join("")}),)})(n).unwrap_or(n.to_string()),].join("\n")}))}
fn main() {
  println!(
    "{}",
    (0..100).fold(String::new(), |a, n| {
      [
        a,
        (|n: i32| {
          (|s: String| (!s.is_empty()).then_some(s))([(3, "Fizz"), (5, "Buzz")].into_iter().fold(
            String::new(),
            |a, (c, s)| {
              [
                a,
                match n % c {
                  0 => s.to_string(),
                  _ => String::new(),
                },
              ]
              .join("")
            },
          ))
        })(n)
        .unwrap_or(n.to_string()),
      ]
      .join("\n")
    })
  )
}
