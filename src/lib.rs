use std::process::Command;
use std::vec::Vec;

use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug, Clone)]
pub struct SwayOutput {
    name: String,
    focused: bool,
    current_workspace: String
}

#[derive(Deserialize, Debug)]
pub struct SwayOutputs(Vec<SwayOutput>);

impl SwayOutputs {
    pub fn get_focused_output(&self) -> Option<SwayOutput> {
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
pub struct SwayWorkspaces(Vec<SwayWorkspace>);

impl SwayWorkspaces {
    pub fn move_to_output(&self, output: &SwayOutput) {
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

pub fn swaymsg<T>(args: Vec<&str>) -> T where T: DeserializeOwned {
    let output = Command::new("swaymsg").args(args).output().expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();

    let sway_data: T = serde_json::from_str(&stdout).unwrap();
    sway_data
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
