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
pub struct Biometric<R: Runtime>(PluginHandle<R>);

#[derive(Serialize)]
struct AuthenticatePayload {
    reason: String,
    #[serde(flatten)]
    options: AuthOptions,
}

impl<R: Runtime> Biometric<R> {
    pub fn status(&self) -> crate::Result<Status> {
        #[cfg(mobile)] {
            self.0.run_mobile_plugin("status", ()).map_err(Into::into)
        }
        #[cfg(not(mobile))]
        {
            Ok(Status {
                is_available: false,
                biometry_type: BiometryType::None,
                error: None,
                error_code: None,
            })
        }
    }

    pub fn authenticate(&self, reason: String, options: AuthOptions) -> crate::Result<()> {
        #[cfg(mobile)] {
	        self.0
	            .run_mobile_plugin("authenticate", AuthenticatePayload { reason, options })
	            .map_err(Into::into)
        }
        #[cfg(not(mobile))]
        {
            Ok(())
        }
    }
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
            let handle = None
            #[cfg(target_os = "android")]
            handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "BiometricPlugin")?;
            #[cfg(target_os = "ios")]
            handle = api.register_ios_plugin(init_plugin_biometric)?;
            app.manage(Biometric(handle));
            Ok(())
        })
        .build()
}
