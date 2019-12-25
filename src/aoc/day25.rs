use super::intcode::Machine;
use std::io::stdin;

fn run_command(machine: &mut Machine, command: String) {
    let mut input: Vec<i64> = Vec::new();

    for c in command.chars().rev() {
        input.push(c as i64);
    }

    machine.run(input);
}

fn reboot(machine: &Machine) -> Machine {
    let mut machine = Machine::init(&machine.positions);
    machine.run(vec![]);
    machine
}

fn rewind(commands: &mut Vec<String>, mut machine: &mut Machine) {
    println!("Rewinding");
    commands.pop();

    for c in commands.iter() {
        run_command(&mut machine, c.to_string());
    }
}

#[allow(dead_code)]
pub fn solve() {
    let initial = Machine::from_file("input/day25.txt");
    let mut commands: Vec<String> = Vec::new();
    let mut machine = Machine::init(&initial.positions);

    machine.run(vec![]);

    // Items in your inventory:
    // - hypercube
    // - festive hat
    // - shell
    // - astronaut ice cream

    loop {
        let bytes: Vec<u8> = machine.values.drain(..).map(|x| x as u8).collect();
        let string = String::from_utf8(bytes).unwrap();
        println!("{}", string);

        // Restart
        if machine.halted {
            println!("Game Over!");
            machine = reboot(&initial);
            rewind(&mut commands, &mut machine);

            continue;
        }

        let mut line = String::new();

        match stdin().read_line(&mut line) {
            Ok(_) => {
                commands.push(line.clone());
                run_command(&mut machine, line);
            }

            Err(error) => {
                println!("Error: {:?}", error);
                break;
            }
        }
    }

    println!("Commands: {:?}", commands);
}
