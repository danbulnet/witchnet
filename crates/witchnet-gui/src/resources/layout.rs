pub const DEFAULT_PANEL_WIDTH: f32 = 210f32;
pub const DEFAULT_PANEL_SCROLL_WIDTH: f32 = 228f32;

#[derive(Debug, Clone, Copy)]
pub(crate) enum CentralPanel {
    Simulation2D,
    Simulation3D,
}

pub(crate) struct Layout {
    pub(crate) central_panel: CentralPanel,

    pub(crate) data: bool,
    pub(crate) appearance: bool,
    pub(crate) simulation_2d: bool,
    pub(crate) simulation_3d: bool,
    pub(crate) sensors: bool,
    pub(crate) neurons: bool,
    pub(crate) connections: bool
}

impl Default for Layout {
    fn default() -> Self {
        Layout { 
            central_panel: CentralPanel::Simulation2D, 

            data: true,
            appearance: false,
            simulation_2d: true,
            simulation_3d: false,
            sensors: true,
            neurons: false,
            connections: false
        }
    }   
}

impl Layout {
    pub(crate) fn simulation_2d_clicked(&mut self) {
        self.simulation_2d = true;
        self.simulation_3d = false;
        self.central_panel = CentralPanel::Simulation2D;
    }

    pub(crate) fn simulation_3d_clicked(&mut self) {
        self.simulation_3d = true;
        self.simulation_2d = false;
        self.central_panel = CentralPanel::Simulation3D;
    }
}