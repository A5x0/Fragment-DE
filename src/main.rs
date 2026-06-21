mod config;

use config::FragmentConfig;
use qmetaobject::*;

fn main() {
    // Load YAML config
    let cfg = FragmentConfig::load();

    // Create QML engine
    let mut engine = QmlEngine::new();

    //
    #[derive(QObject, Default)]
struct ConfigBridge {
    base: qt_base_class!(trait QObject),

    #[qt_signal]
    configChanged: fn(),
}


    fn apply_config(engine: &mut QmlEngine, bridge: &ConfigBridge, cfg: &FragmentConfig) {
    engine.set_property("configWidth".into(), cfg.shell.width.into());
    engine.set_property("configHeight".into(), cfg.shell.height.into());
    engine.set_property("configBackground".into(), QString::from(cfg.shell.background.as_str()).into());
    engine.set_property("configFrameless".into(), cfg.shell.frameless.into());

    (bridge.configChanged)();
}
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::{Arc, Mutex};

let engine = Arc::new(Mutex::new(engine));
let bridge = Arc::new(bridge);

std::thread::spawn({
    let engine = engine.clone();
    let bridge = bridge.clone();

    move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
        watcher.watch("config/config.yaml", RecursiveMode::NonRecursive).unwrap();

        loop {
            if rx.recv().is_ok() {
                if let Ok(new_cfg) = FragmentConfig::load_safe() {
                    let mut engine = engine.lock().unwrap();
                    let bridge = bridge.as_ref();
                    apply_config(&mut engine, bridge, &new_cfg);
                }
            }
        }
    }
});

    impl FragmentConfig {
    pub fn load_safe() -> Result<Self, ()> {
        std::fs::read_to_string("config/config.yaml")
            .ok()
            .and_then(|s| serde_yaml::from_str(&s).ok())
            .ok_or(())
    }
}


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

        Connections {
    target: Config
    function onConfigChanged() {
        root.width = configWidth
        root.height = configHeight
        root.color = configBackground
        root.flags = configFrameless ? Qt.FramelessWindowHint : Qt.Window
    }
}

    "#.into());

    engine.exec();
}
