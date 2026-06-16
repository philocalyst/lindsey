use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::theme::{Theme, ThemeMode};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::Duration,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LindseySettings {
    pub dark_mode: bool,
    pub sidebar_width: f32,
    pub font_size: f32,
}

impl Default for LindseySettings {
    fn default() -> Self {
        Self {
            dark_mode: false,
            sidebar_width: 240.0,
            font_size: 14.0,
        }
    }
}

impl Global for LindseySettings {}

impl LindseySettings {
    pub fn get(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn init(cx: &mut App) {
        let path = settings_path();

        let settings = load_from_file(&path).unwrap_or_default();
        let dark_mode = settings.dark_mode;
        cx.set_global(settings);
        Theme::change(
            if dark_mode {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            None,
            cx,
        );

        // Ensure the settings directory and file exist for the user to edit.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        if !path.exists() {
            if let Ok(json) = serde_json::to_string_pretty(&LindseySettings::default()) {
                std::fs::write(&path, json).ok();
            }
        }

        // Poll for file changes every 2 seconds and apply them live.
        cx.spawn(async move |cx| {
            let mut last_content = std::fs::read_to_string(&path).unwrap_or_default();
            loop {
                let executor = cx.background_executor().clone();
                executor.timer(Duration::from_secs(2)).await;

                if let Ok(content) = std::fs::read_to_string(&path) {
                    if content != last_content {
                        last_content = content.clone();
                        if let Ok(new_settings) =
                            serde_json::from_str::<LindseySettings>(&content)
                        {
                            let dark = new_settings.dark_mode;
                            cx.update(|cx| {
                                cx.set_global(new_settings);
                                Theme::change(
                                    if dark {
                                        ThemeMode::Dark
                                    } else {
                                        ThemeMode::Light
                                    },
                                    None,
                                    cx,
                                );
                            });
                            cx.refresh();
                        }
                    }
                }
            }
        })
        .detach();
    }
}

fn settings_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join("Library/Application Support/lindsey/settings.json")
}

fn load_from_file(path: &PathBuf) -> Option<LindseySettings> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}
