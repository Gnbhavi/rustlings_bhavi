// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::fmt::format;

    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut sender_things: Vec<String> = Vec::new();

        for (string_val, command) in input {
            sender_things.push(match command {
                Command::Uppercase => string_val.to_uppercase(),
                Command::Trim => string_val.trim().to_string(),
                // Command::Append(bar_runs) => {
                //     for _i in 1..=bar_runs {
                //         string_val += "bar";
                //     }
                //     string_val
                // }
                Command::Append(bar_runs) => {
                    let append: String = (1..=bar_runs).map(|_| "bar").collect();
                    string_val + &append
                }
            });
        }
        sender_things
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
