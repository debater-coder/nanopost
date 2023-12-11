use nanopost::*;

fn main() {
    build();

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && args[1] == "dev" {
        serve();
    }
}
