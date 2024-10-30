use std::io;
use colored::Colorize;

fn main() { 
  loop{
    println!("Enter your code's output"); 
    let output: Vec<String> = read_lines();

    println!("Enter the other codes output"); 
    let other_output: Vec<String> = read_lines();
    
    println!("Let's compare (In rust btw)");
  
    match output.len() == other_output.len(){
	false => continue,
	true  => { for i in 0..output.len() {
		     match output[i] == other_output[i]{ 
			false => println!("{}", output[i].red()),			
			true  => println!("{}", output[i].green()),	
		    }} break; },
    } 
  }
}


fn read_lines() -> Vec<String>{
  let stdin = io::stdin();
  let mut output = Vec::new(); 
  for line in stdin.lines() {
    let line = line.unwrap();
    if line.trim() == "exit"{
	break;
    }
    output.push(line);
  }
  output
}
