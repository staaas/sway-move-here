use std::error::Error;
use std::process::Command;
use std::vec::Vec;

use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SwayOutput {
    name: String,
    focused: bool,
    current_workspace: String,
}

impl SwayOutput {
    pub fn get_current_workspace(&self) -> SwayWorkspace {
        let workspace_name = self.current_workspace.clone();
        let workspace_output = self.name.clone();
        SwayWorkspace::new(workspace_name, workspace_output)
    }
}

#[derive(Deserialize, Debug)]
pub struct SwayOutputs(Vec<SwayOutput>);

impl SwayOutputs {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        swaymsg_and_deserialize(vec!["-t", "get_outputs"])
    }

    pub fn get_focused_output(&self) -> Result<SwayOutput, Box<dyn Error>> {
        for output in &self.0 {
            if output.focused {
                return Ok(output.clone());
            }
        }

        let err: Box<Error> = From::from("Cannot determine which output is focused");
        return Err(err);
    }
}

#[derive(Deserialize, Debug)]
pub struct SwayWorkspace {
    name: String,
    output: String,
}

impl SwayWorkspace {
    pub fn new(name: String, output: String) -> Self {
        Self {
            name: name,
            output: output,
        }
    }
    pub fn switch(&self) -> Result<(), Box<dyn Error>> {
        swaymsg(vec!["workspace", &self.name])?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct SwayWorkspaces(Vec<SwayWorkspace>);

impl SwayWorkspaces {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        swaymsg_and_deserialize(vec!["-t", "get_workspaces"])
    }
    pub fn move_to_output(&self, output: &SwayOutput) -> Result<(), Box<dyn Error>> {
        for workspace in &self.0 {
            if workspace.output != output.name {
                let workspace_filter = format!("[workspace=\"{}\"]", workspace.name);
                swaymsg(vec![
                    &workspace_filter,
                    "move",
                    "workspace",
                    "to",
                    "output",
                    &output.name,
                ])?;
            }
        }
        Ok(())
    }
}

fn swaymsg(args: Vec<&str>) -> Result<String, Box<dyn Error>> {
    log::info!("Calling swaymsg with args {:?}", &args);
    let output = Command::new("swaymsg").args(args).output()?;

    match output.status.code() {
        Some(status_code) if status_code == 0 => {
            log::info!("Exit code: {}", &status_code);
        }
        Some(status_code) => {
            let stderr = String::from_utf8(output.stderr).unwrap_or_else(|_| String::new());
            log::error!("Standard error: {}", stderr);
            let err_msg = format!("Command swaymsg failed with exit code {}", &status_code);
            let err: Box<Error> = From::from(err_msg);
            return Err(err);
        }
        None => {
            let err: Box<Error> = From::from("Could not start command swaymsg");
            return Err(err);
        }
    }

    let stdout = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());
    log::debug!("Standard output: {}", stdout);
    Ok(stdout)
}

fn swaymsg_and_deserialize<T>(args: Vec<&str>) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let stdout = swaymsg(args)?;
    let sway_data: T = serde_json::from_str(&stdout)?;
    Ok(sway_data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
