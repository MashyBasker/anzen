#![allow(dead_code)]

use crate::keylist::GPGKey;
use cursive::align::HAlign;
use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};
use cursive::view::Resizable;
use cursive::views::{Dialog, LinearLayout};
use cursive::views::{SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use git2::Repository;
use std::sync::{Arc, Mutex};

/// adds the Name, Email and GPG signing key to the git config
pub fn set_config(repo_path: &str, gpg_key_info: GPGKey) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut config = repo.config()?;

    // add key, value pair in git config file
    config.set_str("user.name", &gpg_key_info.username)?;
    config.set_str("user.email", &gpg_key_info.email.unwrap())?;
    config.set_str("user.signingkey", &gpg_key_info.id)?;
    Ok(())
}

/// add selection tui to choose user to add in config
pub fn choose_signing_key(gpg_info_list: Vec<GPGKey>) -> Result<String, git2::Error> {
    if gpg_info_list.is_empty() {
        return Err(git2::Error::from_str("No GPG keys available"));
    }

    let mut app = Cursive::new();
    app.set_theme(create_theme());
    let selected_id = Arc::new(Mutex::new(String::new()));
    let selected_id_cb = Arc::clone(&selected_id);

    let title = TextView::new("Select Signing Key")
        .h_align(HAlign::Center)
        .style(cursive::theme::Effect::Bold);

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    // display selection options
    for gpgkey in gpg_info_list {
        let display = format!(
            "{} / {}",
            gpgkey.username,
            gpgkey.email.unwrap_or_else(|| "No email".to_string())
        );
        select.add_item(display, gpgkey.id);
    }

    // handle selection
    select.set_on_submit(move |c, v: &String| {
        if let Ok(mut id) = selected_id_cb.lock() {
            *id = v.clone();
        }
        c.quit();
    });

    let layout = LinearLayout::vertical()
        .child(TextView::new("").fixed_height(1))
        .child(title)
        .child(TextView::new("").fixed_height(1))
        .child(select)
        .child(TextView::new("").fixed_height(1));

    app.add_layer(
        Dialog::around(layout)
            .title("User selection")
            .padding_lrtb(5, 5, 2, 2),
    );

    app.run();

    let final_id = match selected_id.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => return Err(git2::Error::from_str("Failed to access selected keys")),
    };

    if final_id.is_empty() {
        Err(git2::Error::from_str("No user selected"))
    } else {
        Ok(final_id)
    }
}

fn create_theme() -> Theme {
    let mut palette = Palette::default();

    // Set some nice colors
    palette[PaletteColor::Background] = Color::TerminalDefault;
    palette[PaletteColor::View] = Color::TerminalDefault;
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::Blue);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::Blue);
    palette[PaletteColor::Tertiary] = Color::RgbLowRes(5, 1, 5); // Subtle purple
    palette[PaletteColor::TitlePrimary] = Color::RgbLowRes(5, 4, 0); // Gold-ish
    palette[PaletteColor::Highlight] = Color::RgbLowRes(5, 5, 0); // Bright yellow
    palette[PaletteColor::HighlightInactive] = Color::RgbLowRes(3, 3, 0); // Darker yellow

    let mut theme = Theme::default();
    theme.palette = palette;
    theme.borders = BorderStyle::Simple;

    theme
}
