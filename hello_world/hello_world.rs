// This is the main function
fn main() {
  // This is an immutable variable.
  let john = "John Doe";

  // println! is a macro for writing to standard output
  println!("Hello world! My name is {}", john);

  // a mutable variable
  let mut jane = "Jane";
  println!("I'm {}", jane);

  jane = "Jane Doe";
  println!("... {}", jane);
}
