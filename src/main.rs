use qmetaobject::*;

fn main() {

    let mut engine = QmlEngine::new();

    // Load the QML window
    engine.load_data(r#"
        import QtQuick 2.15
        import QtQuick.Window 2.15

        Window {
            id: root
            visible: true
            width: 800
            height: 600
            title: "Fragment Shell"

            // Frameless window (like your QWidget flags)
            flags: Qt.FramelessWindowHint

            // Background color (your #202020)
            color: "r#202020"
        }
    "#.into());

    engine.exec();
}
