use std::path::PathBuf;

use bevy::prelude::*;

use crate::resources::data::DataFiles;

#[derive(Debug, Clone)]
pub struct ProgramArgs(pub Vec<String>);

impl From<Vec<String>> for ProgramArgs {
    fn from(value: Vec<String>) -> Self {
        ProgramArgs(value)
    }
}

impl ProgramArgs {
    pub(crate) fn handle_args(args: ResMut<ProgramArgs>, mut data_files_res: ResMut<DataFiles>) {
        if let Some(index) = args.0.iter().position(|x| x == "--data") {
            if index + 1 < args.0.len() {
                let file_path = PathBuf::from(&args.0[index + 1]);
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_path.is_file() && file_name.ends_with(".csv") {
                    DataFiles::load_data(file_path, &mut data_files_res);
                }
            }
        }
    }
}