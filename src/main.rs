use std::{collections::HashMap, process::ExitCode};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThemeFile {
    #[serde(rename(serialize = "icon-theme"))]
    icon_theme: IconTheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IconTheme {
    name: String,
    ui: HashMap<String, String>,
    foldername: HashMap<String, String>,
    filename: HashMap<String, String>,
    extension: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NicerFileTheme {
    #[serde(default)]
    name: String,
    #[serde(default)]
    icons: HashMap<String, NicerIconTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NicerIconTheme {
    #[serde(default)]
    ui: Vec<String>,
    #[serde(default)]
    directory: Vec<String>,
    #[serde(default)]
    file: Vec<String>,
    #[serde(default)]
    extension: Vec<String>,
}

fn main() -> ExitCode {
    let s = match std::fs::read_to_string("icons.toml") {
        Ok(v) => v,
        Err(e) => {
            panic!("{e}");
        }
    };

    let t = match toml::from_str::<NicerFileTheme>(&s) {
        Ok(v) => v,
        Err(e) => {
            panic!("{e}");
        }
    };

    let mut ui = HashMap::new();
    let mut directory = HashMap::new();
    let mut file = HashMap::new();
    let mut extension = HashMap::new();
    for (icon_file, icons) in t.icons {
        for icon in icons.ui {
            ui.insert(icon, format!("icons/{}", icon_file.clone()));
        }
        for icon in icons.directory {
            directory.insert(icon, format!("icons/{}", icon_file.clone()));
        }
        for icon in icons.file {
            file.insert(icon, format!("icons/{}", icon_file.clone()));
        }
        for icon in icons.extension {
            extension.insert(icon, format!("icons/{}", icon_file.clone()));
        }
    }

    let new_t = ThemeFile {
        icon_theme: IconTheme {
            name: t.name.clone(),
            ui,
            foldername: directory,
            filename: file,
            extension,
        },
    };

    let Ok(theme) = toml::to_string_pretty(&new_t) else {
        return ExitCode::FAILURE;
    };

    let Ok(_) = std::fs::write("material.toml", theme) else {
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}
