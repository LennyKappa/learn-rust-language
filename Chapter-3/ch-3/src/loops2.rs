pub fn run(){

  print!("Enter a number:  ");

  let mut input = String::new();

  std::io::stdin().read_line(&mut input)
    .expect("Failed to read input");
  
  let input: u32 = input.trim().parse()
    .expect("Not a number!");
  
  let mut i = 0;

  println!("Even: ");
  
  while i <= input{
    print!("{} ", i);
    i += 2;
  }

  i = 1;

  print!("\nOdd: ");

  while i <= input{
    print!("{} ", i);
    i += 2;
  }

}