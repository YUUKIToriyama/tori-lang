mod repl;
use repl::Repl;

fn main() {
    let mut repl = Repl::new(std::io::stdin(), std::io::stdout());
    repl.start();
}
