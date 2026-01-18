pub mod home;

// Global active screen
#[derive(Debug, PartialEq, Clone)]
pub enum ActiveWindow {
    Home(crate::screens::home::state::HomeWindow),
}
