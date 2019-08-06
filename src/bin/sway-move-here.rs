use std::error::Error;

use docopt::Docopt;
use serde::Deserialize;
use simplelog::{
    Config as LogConfig, LevelFilter as LogLevelFilter, TermLogger, TerminalMode as LogTerminalMode,
};

use sway_move_here::{swaymsg, swaymsg_and_deserialize, SwayOutputs, SwayWorkspaces};

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

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Configure logging subsystem
    let log_level_filter = if args.flag_v {
        LogLevelFilter::Info
    } else {
        LogLevelFilter::Warn
    };
    TermLogger::init(
        log_level_filter,
        LogConfig::default(),
        LogTerminalMode::Mixed,
    )?;

    // Main part of the program
    // 1. Get current state of outputs and workspaces
    let sway_outputs: SwayOutputs = swaymsg_and_deserialize(vec!["-t", "get_outputs"])?;
    let focused_output = sway_outputs.get_focused_output().unwrap();
    let current_workspace_name = focused_output.get_current_workspace_name();
    let sway_workspaces: SwayWorkspaces = swaymsg_and_deserialize(vec!["-t", "get_workspaces"])?;
    // 2. Move all the workspaces to the focused output
    sway_workspaces.move_to_output(&focused_output)?;
    // 3. Switch back to the workspace that was focused before we started moving other workspaces
    swaymsg(vec!["workspace", &current_workspace_name])?;

    Ok(())
}
