pub fn run(){
  let mut i = 1;
  while i < 6 {
    let mut j = 1;
    while j <= i{
      print!("*");
      j += 1;
    }
    print!("\n");
    i += 1
  }
  i -= 1;
  while i > 1{
    i-=1;
    let mut j = 1;
    while j <= i{
      print!("*");
      j += 1;
    }
    print!("\n");
  }
}