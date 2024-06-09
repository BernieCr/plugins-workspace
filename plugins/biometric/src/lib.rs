// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// #![cfg(mobile)]

use serde::Serialize;
use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "app.tauri.biometric";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_biometric);

/// Access to the biometric APIs.
#[cfg(mobile)] {
	pub struct Biometric<R: Runtime>(PluginHandle<R>);

	impl<R: Runtime> Biometric<R> {
        pub fn status(&self) -> crate::Result<Status> {
            self.0.run_mobile_plugin("status", ()).map_err(Into::into)
        }

        pub fn authenticate(&self, reason: String, options: AuthOptions) -> crate::Result<()> {
            self.0
	            .run_mobile_plugin("authenticate", AuthenticatePayload { reason, options })
	            .map_err(Into::into)
        }
    }
}

#[cfg(not(mobile))] {
	pub struct Biometric();

	impl Biometric<> {
        pub fn authenticate(&self, reason: String, options: AuthOptions) -> crate::Result<()> {
           Ok(())
        }
    }
}

#[derive(Serialize)]
struct AuthenticatePayload {
    reason: String,
    #[serde(flatten)]
    options: AuthOptions,
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the biometric APIs.
pub trait BiometricExt<R: Runtime> {
    fn biometric(&self) -> &Biometric<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BiometricExt<R> for T {
    fn biometric(&self) -> &Biometric<R> {
        self.state::<Biometric<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("biometric")
        .setup(|app, api| {
            #[cfg(mobile)] {
	            #[cfg(target_os = "android")]
	            let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "BiometricPlugin")?;
	            #[cfg(target_os = "ios")]
	            let handle = api.register_ios_plugin(init_plugin_biometric)?;
                app.manage(Biometric(handle));
            }
            #[cfg(not(mobile))]
            {
                app.manage(Biometric());
            }
            Ok(())
        })
        .build()
}
