use std::env;
use std::fs;

fn execute(mut program : Vec<usize>, noun : usize, verb : usize) -> usize {
    let mut pc : usize = 0;
    program[1] = noun;
    program[2] = verb;
    while(99 != program[pc])
    {
        let dest = program[pc + 3];
        let lhs = program[pc + 1];
        let rhs = program[pc + 2];
       match &program[pc]
       {
           1 => {
               program[dest] = program[lhs] + program[rhs];
               pc += 4;
           }
           2 => {
               program[dest] = program[lhs] * program[rhs];
               pc += 4;
           }
           99 => {
               break;
           }
           _ => {
               println!("aj");
           }
       } 
    }
    program[0]
}

fn main() {
    let data = fs::read_to_string("data/2")
        .expect("Something went wrong reading the file");
    let mut program : Vec<usize> = data.split(",")
                      .map(|mass_str| mass_str.parse::<usize>().unwrap())
                      .collect();
    for x in &program
    {
        println!("{}", x);
    }
    println!("part 1: {}", execute(program.clone(), 12, 2));
    for i in 0..99
    {
        for j in 0..99
        {
            if(19690720 == execute(program.clone(), i, j))
            {
                println!("noun: {}, verb: {}, answer: {}", i, j, 100 * i + j);
                break;
            }
        }
    }
}