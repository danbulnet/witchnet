use std::{
    path::PathBuf,
    default::Default,
    collections::BTreeMap
};

use rfd::{ MessageDialog, MessageLevel };

use polars::prelude::*;

use bevy_egui::egui::Color32;

use witchnet_common::polars as polars_common;

pub const DATA_PANEL_WIDTH: f32 = 180f32;
pub const DATA_PANEL_SCROLL_WIDTH: f32 = 198f32;

pub const FILE_NAME_OK_COLOR: Color32 = Color32::from_rgb(194, 232, 148);
pub const FILE_NAME_ERR_COLOR: Color32 = Color32::from_rgb(232, 148, 148);

#[derive(Debug, Clone)]
pub(crate) struct SequentialDataFile {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) data_frame: Option<DataFrame>,
    pub(crate) features: BTreeMap<String, bool>,
    pub(crate) rows_limit: usize,
    pub(crate) exequal_sampling: bool
}

#[derive(Debug)]
pub struct SequentialDataFiles{
    pub(crate) current: Option<usize>,
    pub(crate) history: Vec<SequentialDataFile>
}

impl SequentialDataFiles {
    pub(crate) fn current_data_file(&mut self) -> Option<&mut SequentialDataFile> {
        self.history.get_mut(self.current?)
    }
}

impl Default for SequentialDataFiles {
    fn default() -> Self { 
        SequentialDataFiles{ current: None, history: Vec::new() } 
    }
}

impl SequentialDataFiles {
    pub fn load_data(file_path: PathBuf, data_files_res: &mut SequentialDataFiles) {
        let file_name = match file_path.file_name() {
            Some(file_name) => file_name.to_os_string().into_string().ok(),
            None => None
        };
    
        if let Some(file_name) = file_name {
            let mut found = false;
            for (i, data_file) in (&data_files_res.history).into_iter().enumerate() {
                if &data_file.path ==  &file_path {
                    data_files_res.current = Some(i);
                    found = true;
                    break
                }
            }
    
            if !found {
                let data_frame = polars_common::csv_to_dataframe(
                    file_path.as_os_str().to_str().unwrap(), &vec![]
                ).ok();
                if data_frame.is_none() {
                    MessageDialog::new().set_level(MessageLevel::Error)
                        .set_title("file loading error")
                        .set_description(&format!("error converting {} to dataframe", file_name))
                        .show();
                    data_files_res.current = None;
                } else {
                    let mut features: BTreeMap<String, bool> = data_frame.as_ref().unwrap()
                        .get_column_names()
                        .into_iter()
                        .map(|x| (x.to_string(), true))
                        .collect();
                    let nrows = if let Some(df) = &data_frame { df.height() } else { 0 };
                    let data_file = SequentialDataFile { 
                        name: file_name, 
                        path: file_path, 
                        data_frame, 
                        features,
                        rows_limit: nrows,
                        exequal_sampling: false
                    };
                    data_files_res.history.push(data_file);
                    data_files_res.current = Some(data_files_res.history.len() - 1);
                }
            }
        }
    }
}