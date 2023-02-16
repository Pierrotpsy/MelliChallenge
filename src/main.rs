use std::collections::HashMap;
use std::fs;

fn main() {
    let test = fs::read_to_string("input-2.asm").unwrap();

    let test1 : String = "jnz 0 8
mov a 9992
mov b 3
mov c -1
add a c
add b c
jnz b -2
jnz 1 2
mov a 10060
print a
mov a 10
print a".to_string();
    interpreter(test);
}

fn interpreter(program: String) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let split = program.split("\n").collect::<Vec<&str>>();
    let mut i = 0;

    while i < split.len() {
        let instruction = split[i].split_whitespace().collect::<Vec<&str>>();
        match instruction[..] {
            ["mov", x, y] => {
                registers.insert(x, match y.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => {
                        match registers.get(y) {
                            Some(value) => *value,
                            None => {println!("Uninitialized register");return;}
                        }
                    }
                });
            },
            ["add", x, y] => {
                let temp = match y.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => {
                        match registers.get(y) {
                            Some(value) => *value,
                            None => {println!("Uninitialized register");return;}
                        }
                    }
                };;
                let Some(mut value) = registers.get_mut(x) else {println!("Uninitialized register"); return;};
                *value += temp;
            },
            ["print", x] => {
                let Some(value) = registers.get(x) else {println!("Uninitialized register"); return;};
                match char::from_u32(*value as u32) {
                    Some(value) => print!("{}", value),
                    None => println!("Invalid Unicode Character")
                }
            },
            ["jnz", x, y] => {
                let temp = match x.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => {
                        match registers.get(x) {
                            Some(value) => *value,
                            None => {println!("Uninitialized register");return;}
                        }
                    }
                };

                if temp != 0 {
                    i = (i as i32 + y.parse::<i32>().unwrap() - 1) as usize;
                }
            }
            _ => {
                println!("Invalid instruction: {}", split[i]);
                return;
            }
        }
        i += 1;
    }
}

