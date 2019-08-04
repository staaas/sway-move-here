use sway_move_here::{SwayOutputs, SwayWorkspaces, swaymsg};

fn main() {
    let sway_outputs: SwayOutputs = swaymsg(vec!["-t", "get_outputs"]);
    let focused_output = sway_outputs.get_focused_output().unwrap();
    let sway_workspaces : SwayWorkspaces = swaymsg(vec!["-t", "get_workspaces"]);
    sway_workspaces.move_to_output(&focused_output);
}
