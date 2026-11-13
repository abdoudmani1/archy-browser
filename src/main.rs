use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Notebook, Box as GtkBox, Label, Orientation, ReliefStyle, Image};
use gtk::pango::EllipsizeMode;
use webkit2gtk::{CacheModel, HardwareAccelerationPolicy, Settings, WebContext, WebView};
use webkit2gtk::{SettingsExt, WebContextExt, WebViewExt};
use std::env;

const START_PAGE_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>New Tab</title>
    <style>
        body {
            background-color: #11111b;
            color: #cdd6f4;
            font-family: system-ui, -apple-system, sans-serif;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }
        .logo { font-size: 4rem; font-weight: bold; margin-bottom: 2rem; color: #89b4fa; display: flex; align-items: center; }
        form { display: flex; width: 100%; max-width: 600px; }
        input[type="text"] {
            flex: 1;
            padding: 1rem 1.5rem;
            font-size: 1.2rem;
            border: 2px solid #313244;
            border-radius: 24px;
            background-color: #1e1e2e;
            color: #cdd6f4;
            outline: none;
            transition: border-color 0.2s;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }
        input[type="text"]:focus { border-color: #89b4fa; }
    </style>
</head>
<body>
    <div class="logo">
        <img src="archy-logo.png" alt="logo" width="80" height="80" style="margin-right: 20px; border-radius: 16px;">
        Archy Browser
    </div>
    <form action="https://www.google.com/search" method="GET">
        <input type="text" name="q" placeholder="Search on Google" autofocus autocomplete="off">
    </form>
</body>
</html>
"#;

fn parse_input(input: &str) -> String {
    let input = input.trim();
    if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("archy://") || input.starts_with("file://") {
        input.to_string()
    } else if input.contains('.') && !input.contains(' ') {
        format!("https://{}", input)
    } else {
        let query = input.replace(' ', "+");
        format!("https://www.google.com/search?q={}", query)
    }
}

fn create_tab(
    notebook: &Notebook,
    url: &str,
    context: &WebContext,
    settings: &Settings,
) {
    let page_box = GtkBox::new(Orientation::Vertical, 0);

    let nav_bar = GtkBox::new(Orientation::Horizontal, 5);
    nav_bar.set_margin_start(8);
    nav_bar.set_margin_end(8);
    nav_bar.set_margin_top(8);
    nav_bar.set_margin_bottom(8);

    let back_btn = Button::with_label("<");
    let fwd_btn = Button::with_label(">");
    let reload_btn = Button::with_label("↻");
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Search on Google or enter a URL..."));
    url_entry.set_hexpand(true);

    nav_bar.pack_start(&back_btn, false, false, 0);
    nav_bar.pack_start(&fwd_btn, false, false, 0);
    nav_bar.pack_start(&reload_btn, false, false, 0);
    nav_bar.pack_start(&url_entry, true, true, 0);

    let webview = WebView::with_context(context);
    webview.set_settings(settings);
    webview.set_hexpand(true);
    webview.set_vexpand(true);

    page_box.pack_start(&nav_bar, false, false, 0);
    page_box.pack_start(&webview, true, true, 0);
    page_box.show_all();

    let home = env::var("HOME").unwrap_or_else(|_| "/home/me".to_string());
    let base_uri = format!("file://{}/.local/share/icons/hicolor/256x256/apps/", home);

    if url == "archy://start" {
        webview.load_html(START_PAGE_HTML, Some(&base_uri));
    } else {
        webview.load_uri(url);
    }

    let tab_box = GtkBox::new(Orientation::Horizontal, 5);
    
    // Create the globe icon for regular websites
    let tab_icon = Image::from_icon_name(Some("applications-internet-symbolic"), gtk::IconSize::Menu);
    
    // Set the default text to New Tab instead of Start
    let label = Label::new(Some("New Tab"));
    label.set_ellipsize(EllipsizeMode::End);
    label.set_width_chars(15);
    label.set_max_width_chars(30);
    label.set_halign(gtk::Align::Start);
    
    let close_btn = Button::from_icon_name(Some("window-close-symbolic"), gtk::IconSize::Menu);
    close_btn.set_relief(ReliefStyle::None);

    tab_box.pack_start(&tab_icon, false, false, 0);
    tab_box.pack_start(&label, true, true, 0);
    tab_box.pack_start(&close_btn, false, false, 0);
    tab_box.show_all();

    // Hide the icon immediately if this is the start page
    if url == "archy://start" {
        tab_icon.set_visible(false);
    }

    let index = notebook.append_page(&page_box, Some(&tab_box));
    notebook.set_tab_reorderable(&page_box, true);

    let wv_load = webview.clone();
    url_entry.connect_activate(move |entry| {
        let parsed_url = parse_input(entry.text().as_str());
        wv_load.load_uri(&parsed_url);
    });

    let entry_update = url_entry.clone();
    let icon_update = tab_icon.clone();
    webview.connect_uri_notify(move |wv| {
        if let Some(uri) = wv.uri() {
            if uri.as_str().starts_with("archy://start") || uri.as_str().starts_with("file://") {
                entry_update.set_text("");
                icon_update.set_visible(false); // Hide logo on New Tab
            } else {
                entry_update.set_text(uri.as_str());
                icon_update.set_visible(true); // Show logo on websites
            }
        }
    });

    let label_clone = label.clone();
    webview.connect_title_notify(move |wv| {
        if let Some(title) = wv.title() {
            label_clone.set_text(title.as_str());
        }
    });

    let wv_back = webview.clone();
    back_btn.connect_clicked(move |_| {
        if wv_back.can_go_back() { wv_back.go_back(); }
    });

    let wv_fwd = webview.clone();
    fwd_btn.connect_clicked(move |_| {
        if wv_fwd.can_go_forward() { wv_fwd.go_forward(); }
    });

    let wv_reload = webview.clone();
    reload_btn.connect_clicked(move |_| {
        wv_reload.reload();
    });

    let nb_close = notebook.clone();
    let page_close = page_box.clone();
    close_btn.connect_clicked(move |_| {
        if let Some(idx) = nb_close.page_num(&page_close) {
            nb_close.remove_page(Some(idx));
        }
    });

    notebook.set_current_page(Some(index));
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Archy Browser")
        .default_width(1280)
        .default_height(720)
        .build();

    let notebook = Notebook::new();
    notebook.set_scrollable(true);
    
    let new_tab_btn = Button::with_label("+");
    new_tab_btn.set_relief(ReliefStyle::None);
    notebook.set_action_widget(&new_tab_btn, gtk::PackType::End);
    new_tab_btn.show();

    window.add(&notebook);

    let context = WebContext::default().expect("Failed to initialize WebContext");
    context.set_cache_model(CacheModel::DocumentViewer);

    let settings = Settings::new();
    settings.set_enable_webgl(false);
    settings.set_enable_smooth_scrolling(false);
    settings.set_enable_media_stream(false);
    settings.set_hardware_acceleration_policy(HardwareAccelerationPolicy::Never);
    settings.set_allow_file_access_from_file_urls(true);

    let nb_new = notebook.clone();
    let ctx_new = context.clone();
    let set_new = settings.clone();
    new_tab_btn.connect_clicked(move |_| {
        create_tab(&nb_new, "archy://start", &ctx_new, &set_new);
    });

    create_tab(&notebook, "archy://start", &context, &settings);

    window.show_all();
}

fn main() {
    let app = Application::builder()
        .application_id("com.system.archy")
        .build();

    app.connect_activate(build_ui);
    app.run();}
