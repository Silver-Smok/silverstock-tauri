#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

 use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

 use tauri::{AppHandle,Manager,SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};



fn main() {

    /*
    Menu principal de l'application
    */

    let devTool = CustomMenuItem::new("openDevTools".to_string(), "Open DevTools"); // only on run dev

    let edit = Submenu::new("Edition", Menu::new().add_native_item(MenuItem::Copy).add_native_item(MenuItem::Paste).add_native_item(MenuItem::Cut));
    let fenetre = Submenu::new("Fenêtre", Menu::new().add_native_item(MenuItem::CloseWindow).add_native_item(MenuItem::Minimize).add_native_item(MenuItem::Zoom).add_item(devTool).add_item(CustomMenuItem::new("fullScreen".to_string(), "Full Screen")));
    let menu = Menu::new() 
      .add_submenu(edit)
      .add_submenu(fenetre);


    /*
    System tray -> menu contextuel
    */
   
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("visibility-toggle".to_string(), "Hide"))  ;

    /* Je sock le menu contextuel Dans la var Tray */

    let tray = SystemTray::new().with_menu(tray_menu); // 
    tauri::Builder::default()
        .menu(menu)
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .on_menu_event(|event: WindowMenuEvent|
            match event.menu_item_id() {
              "fullScreen" => {
                let window = event.window();
                match window.is_fullscreen() {
                  Ok(true) => window.set_fullscreen(false).unwrap(),
                  Ok(false) => window.set_fullscreen(true).unwrap(),
                  Err(e) => unimplemented!("une erreur qui n'est pas implémentée: " ),
                }
              }
              _ => {}
              
            }
        )
        .on_menu_event(openDevTools)
      

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// fonction qui gère les events du system tray (pour donner la possibilité de cacher/afficher la fenêtre + quitter l'application)

fn on_system_tray_event(app: &AppHandle , event: SystemTrayEvent ) { 
    match event { // quand l'event est un click sur un item du menu 
      SystemTrayEvent::MenuItemClick { id, ..} => { //  on récupère l'id de l'item cliqué
        let item_handle = app.tray_handle().get_item(&id); // on créer un handle pour l'item cliqué
        // dbg!(&id); // On affiche l'id de l'item cliqué dans la console
        match id.as_str() {  // Si l'id de l'item cliqué est égal à "visibility-toggle" alors on cache/affiche la fenêtre
          "visibility-toggle" => {
            let window = app.get_window("main").unwrap(); // on récupère la fenêtre principale
            match window.is_visible() { // si la fenêtre est visible alors on la cache et on change le titre de l'item cliqué en "Show" sinon on l'affiche et on change le titre de l'item cliqué en "Hide"
              Ok(true) => {
                window.hide().unwrap();
                item_handle.set_title("Show").unwrap();
              },
              Ok(false) => {
                window.show().unwrap();
                item_handle.set_title("Hide").unwrap();
              },
              Err(e) => unimplemented!("une erreur qui n'est pas implémentée: " ), // si une erreur est survenue on affiche un message d'erreur
            }
          }
          "quit" => app.exit(0), // si l'id de l'item cliqué est égal à "quit" alors on quitte l'application
          
          _ => {} // si l'id de l'item cliqué est différent de "quit" ou "visibility-toggle" alors on ne fait rien
        }
      }
      _ => {} // si l'event n'est pas un click sur un item du menu alors on ne fait rien
    }
}
  
// fonction qui gère les le devtools Only on run dev

fn openDevTools(event: WindowMenuEvent) {
    #[cfg(any(debug_assertions, feature = "devtools"))]
    #[cfg_attr(doc_cfg, doc(cfg(any(debug_assertions, feature = "devtools"))))]
    match event.menu_item_id() {
        "openDevTools" => {
            let window = event.window();
            window.open_devtools();
        }
        _ => {}
    }
}
  