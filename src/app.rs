use crate::system::SystemInfo;
use crate::network::NetworkInfo;
use crate::ports::PortScanInfo;
use crate::connections::ConnectionInfo;
use crate::ui::popup::Popup;
use crate::actions::{self, Action, AppState};

/// Main application state
///
/// This struct holds all the global state for the application,
/// including the current tab selection and system information.
pub struct App {
    /// System information (CPU, RAM, Disk, GPU, temperatures)
    pub system_info: SystemInfo,
    /// Network information (scans, devices, baseline)
    pub network_info: NetworkInfo,
    /// Port scanning information
    pub port_scan_info: PortScanInfo,
    /// Connection monitoring information
    pub connection_info: ConnectionInfo,
    /// Currently selected tab index
    pub selected_tab: usize,
    /// List of tab names
    pub tabs: Vec<&'static str>,
    /// Optional popup notification (None = no popup)
    pub popup: Option<Popup>,
    /// Selected action index in Actions tab
    pub selected_action: Option<usize>,
    /// Last action result for display
    pub last_action_result: Option<actions::ActionResult>,
}

impl App {
    /// Creates a new App instance with default values
    pub fn new() -> Self {
        Self {
            system_info: SystemInfo::new(),
            network_info: NetworkInfo::new(),
            port_scan_info: PortScanInfo::new(),
            connection_info: ConnectionInfo::new(),
            selected_tab: 0,
            tabs: vec!["Help", "System", "Network", "Ports", "Connections", "Actions"],
            popup: None,
            selected_action: None,
            last_action_result: None,
        }
    }

    /// Refreshes all system information
    pub fn refresh(&mut self) {
        self.system_info.refresh();
        self.connection_info.refresh();
    }

    /// Switches to the next tab (wraps around)
    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
        // Reset action selection when switching tabs
        if self.selected_tab != 4 {
            self.selected_action = None;
        }
    }

    /// Switches to the previous tab (wraps around)
    pub fn prev_tab(&mut self) {
        if self.selected_tab == 0 {
            self.selected_tab = self.tabs.len() - 1;
        } else {
            self.selected_tab -= 1;
        }
        // Reset action selection when switching tabs
        if self.selected_tab != 4 {
            self.selected_action = None;
        }
    }

    /// Shows a popup notification
    pub fn show_popup(&mut self, msg: String) {
        self.popup = Some(Popup::new(msg));
    }

    /// Clears the popup if it should close (after 2 seconds)
    pub fn update_popup(&mut self) {
        if let Some(ref popup) = self.popup {
            if popup.should_close() {
                self.popup = None;
            }
        }
    }

    /// Executes the selected action in the Actions tab
    pub fn execute_selected_action(&mut self) {
        if let Some(idx) = self.selected_action {
            let action = match idx {
                0 => Action::FlushNetworkCache,
                1 => Action::ExportReport,
                2 => Action::CheckInternet,
                3 => Action::SystemUpdate,
                _ => return,
            };
            
            let app_state = AppState {
                system_info: &self.system_info,
                network_info: &self.network_info,
                port_scan_info: &self.port_scan_info,
                connection_info: &self.connection_info,
            };
            
            let result = actions::execute_action(&action, &app_state);
            self.last_action_result = Some(result);
        }
    }
}
