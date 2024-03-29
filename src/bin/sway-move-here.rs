use std::error::Error;

use docopt::Docopt;
use serde::Deserialize;
use simplelog::{
    Config as LogConfig, LevelFilter as LogLevelFilter, TermLogger, TerminalMode as LogTerminalMode,
};

use sway_move_here::{SwayOutputs, SwayWorkspaces};

const USAGE: &'static str = "
Move all the active workspaces to the focused output in Sway window manager.

Usage:
  sway-move-here [-v...]
  sway-move-here (-h | --help)

Options:
  -v...         Verbosity level (from 0 to 2).
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_v: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Configure logging subsystem
    let log_level_filter = match args.flag_v {
        0 => LogLevelFilter::Warn,
        1 => LogLevelFilter::Info,
        _ => LogLevelFilter::Debug,
    };
    TermLogger::init(
        log_level_filter,
        LogConfig::default(),
        LogTerminalMode::Mixed,
    )?;

    // Main part of the program
    // 1. Get current state of outputs and workspaces
    let sway_outputs = SwayOutputs::get()?;
    let focused_output = sway_outputs.get_focused_output()?;
    let current_workspace = focused_output.get_current_workspace();
    let sway_workspaces = SwayWorkspaces::get()?;
    // 2. Move all the workspaces to the focused output
    sway_workspaces.move_to_output(&focused_output)?;
    // 3. Switch back to the workspace that was focused before we started moving other workspaces
    current_workspace.switch()?;

    Ok(())
}
