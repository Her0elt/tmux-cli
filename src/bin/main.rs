use tmux_cli::cli::Cli;

fn main() {
    let cli: Cli = std::env::args().try_into().unwrap();
    cli.execute_command();
}
