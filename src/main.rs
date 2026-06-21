mod config;

use config::FragmentConfig;
use qmetaobject::*;

fn main() {
    // Load YAML config
    let cfg = FragmentConfig::load();

    // Create QML engine
    let mut engine = QmlEngine::new();

    // Expose config values directly as QML context properties
    engine.set_property("configWidth".into(), cfg.shell.width.into());
    engine.set_property("configHeight".into(), cfg.shell.height.into());
    engine.set_property("configBackground".into(), QString::from(cfg.shell.background.as_str()).into());
    engine.set_property("configFrameless".into(), cfg.shell.frameless.into());

    // Load QML
    engine.load_data(r#"
        import QtQuick 2.15
        import QtQuick.Window 2.15

        Window {
            visible: true
            width: configWidth
            height: configHeight
            color: configBackground
            flags: configFrameless ? Qt.FramelessWindowHint : Qt.Window
        }
    "#.into());

    engine.exec();
}
