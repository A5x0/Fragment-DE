mod config;

use config::FragmentConfig;
use qmetaobject::*;

#[derive(QObject, Default)]
struct ConfigBridge {
    base: qt_base_class!(trait QObject),

    #[qt_property]
    background: QString,

    #[qt_property]
    width: u32,

    #[qt_property]
    height: u32,

    #[qt_property]
    frameless: bool,
}

fn main() {
    // Load YAML config
    let cfg = FragmentConfig::load();

    // Create QObject bridge
    let mut bridge = ConfigBridge::default();
    bridge.background = QString::from(cfg.shell.background.as_str());
    bridge.width = cfg.shell.width;
    bridge.height = cfg.shell.height;
    bridge.frameless = cfg.shell.frameless;

    // Pin the QObject
    let bridge = QObjectBox::new(bridge);

    // Create QML engine
    let mut engine = QmlEngine::new();
    engine.set_object_property("Config".into(), bridge.pinned());

    // Load QML
    engine.load_data(r#"
        import QtQuick 2.15
        import QtQuick.Window 2.15

        Window {
            visible: true
            width: Config.width
            height: Config.height
            color: Config.background
            flags: Config.frameless ? Qt.FramelessWindowHint : Qt.Window
        }
    "#.into());

    engine.exec();
}
