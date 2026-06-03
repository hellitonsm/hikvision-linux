//! Exit dialog. Asks for confirmation, then `NET_DVR_Cleanup` and `exit(0)`.

use crate::public::module::{Module, ModuleGroup};

#[derive(Debug, Default)]
pub struct ExitDemo;

impl ExitDemo {
    pub fn new() -> Self { Self }
    pub fn confirm_and_exit(&self) {
        log::info!("User confirmed exit; calling NET_DVR_Cleanup");
        crate::sdk::cleanup();
        std::process::exit(0);
    }
}

impl Module for ExitDemo {
    fn id(&self) -> &'static str { "exit" }
    fn label(&self) -> &'static str { "Exit" }
    fn group(&self) -> ModuleGroup { ModuleGroup::Exit }
}
