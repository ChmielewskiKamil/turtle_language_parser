#[derive(Debug, PartialEq)]
pub enum Command {
    PenSelect(u32),
    PenDown,
    PenUp,
    DrawWest(u32),
    DrawEast(u32),
    DrawNorth(u32),
    DrawSouth(u32),
}

fn parse_single_command(command: &str) -> Result<Command, &'static str> {
    // tokens will be used to derive the command and a parameter from str command
    // the command is formatted like this "P 2"
    // that's why it is necessary to split two tokens on the whitespace
    let tokens: Vec<&str> = command.split_whitespace().collect();
    // first char is the command type
    // as tokens[0] is of type string and we are interested in chars
    // we need to convert it with chars()
    let c = tokens[0].chars().next().unwrap();
    // the integer after the command is the parameter
    let parameter = tokens.get(1).and_then(|s| s.parse().ok());
    match c {
        'P' => Ok(Command::PenSelect(parameter.unwrap())),
        'D' => Ok(Command::PenDown),
        'U' => Ok(Command::PenUp),
        'S' => Ok(Command::DrawSouth(parameter.unwrap())),
        'N' => Ok(Command::DrawNorth(parameter.unwrap())),
        'W' => Ok(Command::DrawWest(parameter.unwrap())),
        'E' => Ok(Command::DrawEast(parameter.unwrap())),
        _ => Err("Could not parse the command."),
    }
}

#[cfg(test)]
mod single_command_parsing_tests {
    use super::{parse_single_command, Command};

    #[test]
    fn it_should_parse_pen_select() {
        assert_eq!(parse_single_command("P 2"), Ok(Command::PenSelect(2)));
    }

    #[test]
    fn it_should_parse_pen_down() {
        assert_eq!(parse_single_command("D"), Ok(Command::PenDown));
    }

    #[test]
    fn it_should_parse_with_trailing_spaces() {
        assert_eq!(parse_single_command(" P 2 "), Ok(Command::PenSelect(2)));
    }

    #[test]
    fn it_should_parse_pen_up() {
        assert_eq!(parse_single_command("U"), Ok(Command::PenUp));
    }

    #[test]
    fn it_should_parse_draw_south() {
        assert_eq!(parse_single_command("S 5"), Ok(Command::DrawSouth(5)));
    }

    #[test]
    fn it_should_parse_draw_north() {
        assert_eq!(parse_single_command("N 10"), Ok(Command::DrawNorth(10)));
    }

    #[test]
    fn it_should_parse_draw_west() {
        assert_eq!(parse_single_command("W 20"), Ok(Command::DrawWest(20)));
    }

    #[test]
    fn it_should_parse_draw_east() {
        assert_eq!(parse_single_command("E 35"), Ok(Command::DrawEast(35)));
    }
}

pub fn parse_input_commands(user_input: &str) -> Result<Vec<Command>, &'static str> {
    user_input.split('\n').map(parse_single_command).collect()
}

#[cfg(test)]
mod user_input_parsing_tests {
    use super::{parse_input_commands, Command};

    #[test]
    fn it_should_parse_single_command() {
        assert_eq!(parse_input_commands("N 1"), Ok(vec![Command::DrawNorth(1)]));
    }

    #[test]
    fn it_should_parse_two_commands() {
        assert_eq!(
            parse_input_commands("P 2\nE 5"),
            Ok(vec![Command::PenSelect(2), Command::DrawEast(5)])
        );
    }
}

fn print_path_for_commands(user_input: &str) -> String {
    let commands = parse_input_commands(user_input).unwrap();

    let mut turtle_path = String::new();

    for command in commands {
        match command {
            Command::DrawNorth(x) => turtle_path.push_str(&format!("Draw north {} cm\n", x)),
            Command::DrawSouth(x) => println!("Draw south {} cm", x),
            _ => todo!(),
        }
    }

    turtle_path
}

#[cfg(test)]
mod path_printing_tests {
    use super::print_path_for_commands;

    #[test]
    fn it_should_print_path_for_draw_north() {
        assert_eq!(print_path_for_commands("N 1"), "Draw north 1 cm\n");
    }

    #[test]
    fn it_should_print_path_for_draw_south() {
        assert_eq!(print_path_for_commands("S 5"), "Draw south 5 cm\n");
    }
}
