use crate::modint::ModInt;
use crate::parser::BF;
use crate::tape::Tape;

pub fn interpreter(
    lst: &[BF],
    mut tape: &mut Tape,
    mut input: &mut (Option<&String>, usize),
    show_tape: bool,
) -> String {
    let mut output = String::new();
    let empty = String::new();
    let input_str = input.0.unwrap_or_else(|| &empty);
    for i in lst {
        match i {
            BF::Add => tape.change(tape.take_val() + ModInt(1)),
            BF::Sub => tape.change(tape.take_val() - ModInt(1)),
            BF::Left => tape.move_left(),
            BF::Right => tape.move_right(),
            BF::Input => {
                if input_str.len() > input.1 {
                    tape.change(ModInt(input_str.bytes().nth(input.1).unwrap()));
                    input.1 += 1;
                }
            }
            BF::Output => output.push_str(&tape.take_val().to_string()),
            BF::Loop(l) => {
                while tape.take_val() != ModInt(0) {
                    output.push_str(&interpreter(&l, &mut tape, &mut input, show_tape));
                }
            }
        }
        if show_tape {
            println!("{:?}", tape);
        }
    }
    output
}
