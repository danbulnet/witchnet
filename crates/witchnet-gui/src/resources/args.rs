use std::path::PathBuf;

use bevy::prelude::*;

use crate::resources::{
    tabular_data::TabularDataFiles,
    sequential_data::SequentialDataFiles
};

#[derive(Debug, Clone)]
pub struct ProgramArgs(pub Vec<String>);

impl From<Vec<String>> for ProgramArgs {
    fn from(value: Vec<String>) -> Self {
        ProgramArgs(value)
    }
}

impl ProgramArgs {
    pub(crate) fn handle_args(
        args: ResMut<ProgramArgs>, 
        mut tabular_data_files_res: ResMut<TabularDataFiles>,
        mut sequential_data_files_res: ResMut<SequentialDataFiles>,
    ) {
        if let Some(index) = args.0.iter().position(|x| x == "--data") {
            if index + 1 < args.0.len() {
                let file_path = PathBuf::from(&args.0[index + 1]);
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_path.is_file() && file_name.ends_with(".csv") {
                    TabularDataFiles::load_data(file_path.clone(), &mut tabular_data_files_res);
                    SequentialDataFiles::load_data(file_path, &mut sequential_data_files_res);
                }
            }
        }
    }
}