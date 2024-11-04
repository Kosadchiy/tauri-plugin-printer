use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Printer;
#[cfg(mobile)]
use mobile::Printer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the printer APIs.
pub trait PrinterExt<R: Runtime> {
  fn printer(&self) -> &Printer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PrinterExt<R> for T {
  fn printer(&self) -> &Printer<R> {
    self.state::<Printer<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("printer")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let printer = mobile::init(app, api)?;
      #[cfg(desktop)]
      let printer = desktop::init(app, api)?;
      app.manage(printer);
      Ok(())
    })
    .build()
}
