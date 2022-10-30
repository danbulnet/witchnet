pub const DEFAULT_PANEL_WIDTH: f32 = 210f32;
pub const DEFAULT_PANEL_SCROLL_WIDTH: f32 = 228f32;

#[derive(Debug, Clone, Copy)]
pub(crate) enum LeftPanel {
    Data,
    Appearance
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CentralPanel {
    Simulation2D,
    Simulation3D,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RightPanel {
    Sensors,
    Neurons,
    Connections
}

pub(crate) struct Layout {
    pub(crate) left_panel: Option<LeftPanel>,
    pub(crate) central_panel: CentralPanel,
    pub(crate) right_panel: Option<RightPanel>,

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
            left_panel: Some(LeftPanel::Data),
            central_panel: CentralPanel::Simulation2D, 
            right_panel: Some(RightPanel::Sensors),

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
    pub(crate) fn data_clicked(&mut self) {
        self.appearance = false;
        self.left_panel = if self.data { Some(LeftPanel::Data) } else { None };
    }

    pub(crate) fn appearance_clicked(&mut self) {
        self.data = false;
        self.left_panel = if self.appearance { Some(LeftPanel::Appearance) } else { None };
    }

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

    pub(crate) fn sensors_clicked(&mut self) {
        self.neurons = false;
        self.connections = false;
        self.right_panel = if self.sensors { Some(RightPanel::Sensors) } else { None };
    }

    pub(crate) fn neurons_clicked(&mut self) {
        self.sensors = false;
        self.connections = false;
        self.right_panel = if self.neurons { Some(RightPanel::Neurons) } else { None };
    }

    pub(crate) fn connections_clicked(&mut self) {
        self.sensors = false;
        self.neurons = false;
        self.right_panel = if self.connections { Some(RightPanel::Connections) } else { None };
    }
}