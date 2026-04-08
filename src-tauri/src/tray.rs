use crate::credential;
use crate::git;
use crate::storage::Storage;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    update_tray_menu(app)?;
    Ok(())
}

pub fn update_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let storage = app.state::<Storage>();
    let accounts = storage.get_accounts();

    let mut menu_builder = MenuBuilder::new(app);

    // Header
    let header = MenuItemBuilder::with_id("header", "Git Account Switcher")
        .enabled(false)
        .build(app)?;
    menu_builder = menu_builder.item(&header);
    menu_builder = menu_builder.separator();

    // Current active account
    let active = accounts.iter().find(|a| a.is_active);
    let status_text = if let Some(a) = active {
        format!("Active: {} ({})", a.display_name, a.host)
    } else {
        "No active account".to_string()
    };
    let status = MenuItemBuilder::with_id("status", &status_text)
        .enabled(false)
        .build(app)?;
    menu_builder = menu_builder.item(&status);
    menu_builder = menu_builder.separator();

    // Account list for quick switching
    for account in &accounts {
        let label = if account.is_active {
            format!("✓ {}", account.display_name)
        } else {
            format!("  {}", account.display_name)
        };
        let item = MenuItemBuilder::with_id(format!("switch_{}", account.id), &label)
            .enabled(!account.is_active)
            .build(app)?;
        menu_builder = menu_builder.item(&item);
    }

    if !accounts.is_empty() {
        menu_builder = menu_builder.separator();
    }

    // Show window
    let show = MenuItemBuilder::with_id("show", "Open Window")
        .build(app)?;
    menu_builder = menu_builder.item(&show);

    // Quit
    let quit = MenuItemBuilder::with_id("quit", "Quit")
        .build(app)?;
    menu_builder = menu_builder.item(&quit);

    let menu = menu_builder.build()?;

    // Try to get existing tray icon, or create a new one
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(menu))?;
        let tooltip = if let Some(a) = active {
            format!("GAS - {}", a.display_name)
        } else {
            "Git Account Switcher".to_string()
        };
        tray.set_tooltip(Some(&tooltip))?;
    } else {
        let png_data = include_bytes!("../icons/32x32.png");
        let decoder = image::codecs::png::PngDecoder::new(std::io::Cursor::new(png_data))
            .expect("Failed to decode icon");
        use image::ImageDecoder;
        let (w, h) = decoder.dimensions();
        let mut rgba = vec![0u8; decoder.total_bytes() as usize];
        decoder.read_image(&mut rgba).expect("Failed to read icon");
        let icon = Image::new_owned(rgba, w, h);

        let tooltip = if let Some(a) = active {
            format!("GAS - {}", a.display_name)
        } else {
            "Git Account Switcher".to_string()
        };

        TrayIconBuilder::with_id("main-tray")
            .icon(icon)
            .menu(&menu)
            .tooltip(&tooltip)
            .on_menu_event(move |app, event| {
                let id = event.id().as_ref();
                if id == "show" {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                } else if id == "quit" {
                    app.exit(0);
                } else if let Some(account_id) = id.strip_prefix("switch_") {
                    handle_tray_switch(app, account_id);
                }
            })
            .build(app)?;
    }

    Ok(())
}

fn handle_tray_switch(app: &AppHandle, account_id: &str) {
    let storage = app.state::<Storage>();
    let account = match storage.get_account(account_id) {
        Some(a) => a,
        None => return,
    };

    let token = match &account.auth_token {
        Some(t) => t.clone(),
        None => return,
    };

    // Switch credential
    let _ = credential::delete_credential(&account.host);
    let _ = credential::write_credential(&account.host, &account.git_username, &token);
    let _ = git::set_global_user(&account.git_username, &account.git_email);
    storage.set_active(account_id, &account.host);

    // Update tray menu to reflect new state
    let _ = update_tray_menu(app);

    // Notify the frontend to refresh
    let _ = app.emit("account-switched", account_id);
}
