use std::process::Command;
use std::vec::Vec;

use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug, Clone)]
struct SwayOutput {
    name: String,
    focused: bool,
    current_workspace: String
}

#[derive(Deserialize, Debug)]
struct SwayOutputs(Vec<SwayOutput>);

impl SwayOutputs {
    fn get_focused_output(&self) -> Option<SwayOutput> {
        for output in &self.0 {
            if output.focused {
                return Some(output.clone());
            }
        }
        None
    }
}

#[derive(Deserialize, Debug)]
struct SwayWorkspace {
    name: String,
    output: String,
}

#[derive(Deserialize, Debug)]
struct SwayWorkspaces(Vec<SwayWorkspace>);

impl SwayWorkspaces {
    fn move_to_output(&self, output: &SwayOutput) {
        for workspace in &self.0 {
            if workspace.output != output.name {
                let workspace_filter = format!("[workspace=\"{}\"]", workspace.name);
                Command::new("swaymsg")
                    .args(&[&workspace_filter, "move", "workspace", "to", "output", &output.name])
                    .output().unwrap();
            }
        }
    }
}

fn swaymsg<T>(args: Vec<&str>) -> T where T: DeserializeOwned {
    let output = Command::new("swaymsg").args(args).output().expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();

    let sway_data: T = serde_json::from_str(&stdout).unwrap();
    sway_data
}

fn main() {
    let sway_outputs: SwayOutputs = swaymsg(vec!["-t", "get_outputs"]);
    let focused_output = sway_outputs.get_focused_output().unwrap();
    let sway_workspaces : SwayWorkspaces = swaymsg(vec!["-t", "get_workspaces"]);
    sway_workspaces.move_to_output(&focused_output);
}
