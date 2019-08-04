use docopt::Docopt;
use serde::Deserialize;

use sway_move_here::{SwayOutputs, SwayWorkspaces, swaymsg_and_deserialize};

const USAGE: &'static str = "
Move all the active workspaces to the focused output in Sway window manager.

Usage:
  sway-move-here [-v]
  sway-move-here (-h | --help)

Options:
  -v            Verbose output.
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_v: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let sway_outputs: SwayOutputs = swaymsg_and_deserialize(vec!["-t", "get_outputs"]);
    let focused_output = sway_outputs.get_focused_output().unwrap();
    let sway_workspaces : SwayWorkspaces = swaymsg_and_deserialize(vec!["-t", "get_workspaces"]);
    sway_workspaces.move_to_output(&focused_output);
}
