use memoize::memoize;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", validate(input, 96979989692495));
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn from_str(input: &str) -> Option<Variable> {
        match input {
            "w" => Some(Variable::W),
            "x" => Some(Variable::X),
            "y" => Some(Variable::Y),
            "z" => Some(Variable::Z),
            _ => None,
        }
    }

    fn get_value(&self, w: i64, x: i64, y: i64, z: i64) -> i64 {
        match self {
            Variable::W => w,
            Variable::X => x,
            Variable::Y => y,
            Variable::Z => z,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Source {
    Variable(Variable),
    Value(i64),
}

impl Source {
    fn from_str(input: &str) -> Source {
        let variable = Variable::from_str(input);
        if let Some(var) = variable {
            return Source::Variable(var);
        }
        Source::Value(input.parse::<i64>().unwrap())
    }

    fn get_value(&self, w: i64, x: i64, y: i64, z: i64) -> i64 {
        match self {
            Source::Variable(var) => var.get_value(w, x, y, z),
            Source::Value(val) => *val,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Command {
    INP(Variable),
    ADD(Variable, Source),
    MUL(Variable, Source),
    DIV(Variable, Source),
    MOD(Variable, Source),
    EQL(Variable, Source),
}

impl Command {
    fn print(&self) {
        println!(
            "{}",
            match self {
                Command::INP(_) => "INP",
                Command::ADD(_, _) => "ADD",
                Command::MUL(_, _) => "MUL",
                Command::DIV(_, _) => "DIV",
                Command::MOD(_, _) => "MOD",
                Command::EQL(_, _) => "EQL",
            }
        )
    }
}

fn str_to_command(input: &str) -> Command {
    let split = input.split(' ').collect::<Vec<&str>>();
    match split[0] {
        "inp" => Command::INP(Variable::from_str(split[1]).unwrap()),
        "add" => Command::ADD(
            Variable::from_str(split[1]).unwrap(),
            Source::from_str(split[2]),
        ),
        "mul" => Command::MUL(
            Variable::from_str(split[1]).unwrap(),
            Source::from_str(split[2]),
        ),
        "div" => Command::DIV(
            Variable::from_str(split[1]).unwrap(),
            Source::from_str(split[2]),
        ),
        "mod" => Command::MOD(
            Variable::from_str(split[1]).unwrap(),
            Source::from_str(split[2]),
        ),
        "eql" => Command::EQL(
            Variable::from_str(split[1]).unwrap(),
            Source::from_str(split[2]),
        ),
        _ => {
            panic!()
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .split('\n')
        .map(|line| str_to_command(line))
        .collect::<Vec<Command>>()
}

#[memoize]
fn execute_command(
    command: Command,
    current_input: i64,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
) -> (i64, i64, i64, i64, bool) {
    let mut w = w;
    let mut x = x;
    let mut y = y;
    let mut z = z;
    let mut input_read = false;
    match command {
        Command::INP(var) => {
            match var {
                Variable::W => w = current_input,
                Variable::X => x = current_input,
                Variable::Y => y = current_input,
                Variable::Z => z = current_input,
            }
            input_read = true;
        }
        Command::ADD(var, source) => {
            let result = var.get_value(w, x, y, z) + source.get_value(w, x, y, z);
            match var {
                Variable::W => w = result,
                Variable::X => x = result,
                Variable::Y => y = result,
                Variable::Z => z = result,
            }
        }
        Command::MUL(var, source) => {
            let result = var.get_value(w, x, y, z) * source.get_value(w, x, y, z);
            match var {
                Variable::W => w = result,
                Variable::X => x = result,
                Variable::Y => y = result,
                Variable::Z => z = result,
            }
        }
        Command::DIV(var, source) => {
            let result = var.get_value(w, x, y, z) / source.get_value(w, x, y, z);
            match var {
                Variable::W => w = result,
                Variable::X => x = result,
                Variable::Y => y = result,
                Variable::Z => z = result,
            }
        }
        Command::MOD(var, source) => {
            let result = var.get_value(w, x, y, z) % source.get_value(w, x, y, z);
            match var {
                Variable::W => w = result,
                Variable::X => x = result,
                Variable::Y => y = result,
                Variable::Z => z = result,
            }
        }
        Command::EQL(var, source) => {
            let comparison = var.get_value(w, x, y, z) == source.get_value(w, x, y, z);
            let result = if comparison { 1 } else { 0 };
            match var {
                Variable::W => w = result,
                Variable::X => x = result,
                Variable::Y => y = result,
                Variable::Z => z = result,
            }
        }
    }
    (w, x, y, z, input_read)
}

fn execute(commands: &Vec<Command>, inputs: Vec<i64>) -> (i64, i64, i64, i64) {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut input_index = 0;
    for command in commands {
        let result = execute_command((*command).clone(), inputs[input_index.min(13)], w, x, y, z);
        w = result.0;
        x = result.1;
        y = result.2;
        z = result.3;
        if result.4 {
            input_index += 1;
        }
    }
    (w, x, y, z)
}

fn part_1(input: &str) -> i64 {
    let commands = parse_input(input);
    let max_model_number = 99999999999999;
    let min_model_number = 11111111111111;
    for model_number in (min_model_number..=max_model_number).rev() {
        if model_number.to_string().contains("0") {
            continue;
        } else {
            let input_vec = model_number
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect();
            if execute(&commands, input_vec).3 == 0 {
                return model_number;
            }
        }
    }
    -1
}

fn validate(input: &str, model_number: i64) -> i64 {
    let commands = parse_input(input);
            let input_vec = model_number
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect();
          execute(&commands, input_vec).3
}
