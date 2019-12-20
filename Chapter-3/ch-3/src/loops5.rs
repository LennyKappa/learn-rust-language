pub fn run(){

  // Take in input
  print!("Enter a number:  ");

  let mut input = String::new();

  std::io::stdin().read_line(&mut input)
    .expect("Failed to read input");

  let input : u32 = input.trim().parse()
    .expect("NaN");
  
  // Make our table
  let mut x = 1;
  let mut y = 1;

  while x <= input{
    
    while y <= input{
      print!("{:^5}", x*y);
      y += 1;
    }
    print!("\n");
    y = 1;
    x += 1;
  }
}