mod interpreter;
mod parse;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!(
            "Wrong number of arguments; expected 1, got {}",
            args.len() - 1
        );
    }
    let file = std::fs::read_to_string(args[1].clone()).expect("Could not read file");
    interpreter::run(parse::parse(file).expect("Could not parse file"))
        .expect("Could not run file");
}
