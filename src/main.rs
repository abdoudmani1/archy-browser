use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use std::env;
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use wry::WebViewBuilderExtWindows; // Import the Windows-specific extension

// ... inside your main function ...

// 1. Create a path to the user's writable Local AppData folder
let mut data_dir = env::temp_dir();
if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
    data_dir = PathBuf::from(local_app_data);
}
data_dir.push("ArchyBrowser"); // This creates %LOCALAPPDATA%\ArchyBrowser

// 2. Apply the data directory to your WebViewBuilder
#[cfg(target_os = "windows")]
let builder = wry::WebViewBuilder::new(&window)
    .with_data_directory(data_dir);

#[cfg(not(target_os = "windows"))]
let builder = wry::WebViewBuilder::new(&window);

let webview = builder
    .with_url("https://duckduckgo.com") // Replace with your actual URL/HTML
    .build()?;
    
fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Archy Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    // Pass &window into new() instead of build()
    let _webview = WebViewBuilder::new(&window)
        .with_url("https://www.google.com")
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
