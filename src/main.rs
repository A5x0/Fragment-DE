use qmetaobject::*;
use qt_widgets::QApplication;
use qt_widgets::QWidget;
use qt_widgets::QColor;

fn main() {
    // Initialize Qt application
    QApplication::init(|_app| {
        // Create the shell window
        let mut window = QWidget::new_0a();

        // Set window title (temporary)
        window.set_window_title(&QString::from("Fragment Shell"));

        // Make it frameless (Fragment controls its own chrome)
        window.set_window_flags(
            qt_widgets::qt_core::qt::WindowType::FramelessWindowHint.into()
        );

        // Set background color (temporary hardcoded, will come from config)
        let color = QColor::from_rgb_3a(32, 32, 32, 255); // #202020
        let palette = window.palette();
        palette.set_color(qt_widgets::qt_core::q_palette::ColorRole::Window, &color);
        window.set_palette(&palette);
        window.set_auto_fill_background(true);

        // Resize to something visible
        window.resize_2a(800, 600);

        // Show the window (of course)
        window.show();

        // Enter Qt event loop
        QApplication::exec()
    });
}

