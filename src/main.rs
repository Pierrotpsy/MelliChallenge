use std::collections::HashMap;
use std::fs;

fn main() {
    let test = fs::read_to_string("input-1.asm").unwrap();
    interpreter(test);
}

fn interpreter(program: String) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let split = program.split("\n");

    for inst in split {
        let instruction = inst.split_whitespace().collect::<Vec<&str>>();
        match instruction[..] {
            ["mov", x, y] => {registers.insert(x, match y.parse::<i32>() {
                Ok(value) => value,
                Err(_) => registers[y]
            });},
            ["add", x, y] => {
                let temp = match y.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => registers[y]
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
            _ => {
                println!("Invalid instruction: {}", inst);
                return;
            }
        }
    }
}

