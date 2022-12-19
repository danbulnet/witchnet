pub const DEFAULT_PANEL_WIDTH: f32 = 210f32;
pub const DEFAULT_PANEL_SCROLL_WIDTH: f32 = 228f32;

#[derive(Debug, Clone, Copy)]
pub(crate) enum CentralPanel {
    MAGDS2D,
    MAGDS3D,
    SequentialModel2D,
    FlexPoints,
}

pub(crate) struct Layout {
    pub(crate) central_panel: CentralPanel,

    pub(crate) tabular_data: bool,
    pub(crate) sequential_data: bool,
    pub(crate) appearance: bool,
    pub(crate) magds_2d: bool,
    pub(crate) magds_3d: bool,
    pub(crate) sequential_model_2d: bool,
    pub(crate) flex_points: bool,
    pub(crate) sensors: bool,
    pub(crate) neurons: bool,
    pub(crate) connections: bool
}

impl Default for Layout {
    fn default() -> Self {
        Layout { 
            central_panel: CentralPanel::MAGDS2D, 

            tabular_data: false,
            sequential_data: true,
            appearance: false,
            magds_2d: true,
            magds_3d: false,
            sequential_model_2d: false,
            flex_points: false,
            sensors: true,
            neurons: false,
            connections: false
        }
    }   
}

impl Layout {
    pub(crate) fn magds_2d_clicked(&mut self) {
        self.magds_2d = true;
        self.magds_3d = false;
        self.sequential_model_2d = false;
        self.flex_points = false;
        self.central_panel = CentralPanel::MAGDS2D;
    }

    pub(crate) fn magds_3d_clicked(&mut self) {
        self.magds_2d = false;
        self.magds_3d = true;
        self.sequential_model_2d = false;
        self.flex_points = false;
        self.central_panel = CentralPanel::MAGDS3D;
    }
    
    pub(crate) fn sequential_model_2d_clicked(&mut self) {
        self.magds_3d = false;
        self.magds_2d = false;
        self.sequential_model_2d = true;
        self.flex_points = false;
        self.central_panel = CentralPanel::SequentialModel2D;
    }

    pub(crate) fn flex_points_clicked(&mut self) {
        self.magds_3d = false;
        self.magds_2d = false;
        self.sequential_model_2d = false;
        self.flex_points = true;
        self.central_panel = CentralPanel::FlexPoints;
    }
}