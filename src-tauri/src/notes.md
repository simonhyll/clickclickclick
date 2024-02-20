#[tauri::command]
fn hotkey() {
    std::thread::spawn(move || {
        let manager = GlobalHotKeyManager::new().unwrap();
        let hotkey = HotKey::new(Some(Modifiers::FN), Code::F1);
        manager.register(hotkey).expect("Failed to register hotkey");
        let mut toggle = false;
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                match event.id {
                    _hotkey if event.state == HotKeyState::Pressed => {
                        toggle = !toggle;
                    }
                    _ => {}
                };
            };
            while toggle {
                if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                    if event.state == HotKeyState::Pressed {
                        toggle = !toggle;
                        break;
                    };
                } else {
                    println!("Hotkey pressed");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                };
            };
        };
    });
}