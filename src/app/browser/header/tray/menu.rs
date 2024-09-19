use gtk::{
    gio,
    MenuButton
};

pub fn new() -> MenuButton
{
    let menu = MenuButton::builder()

        .tooltip_text(
            "Menu"
        )

        .build();

    let model = gio::Menu::new();

        let model_tab = gio::Menu::new();

            model_tab.append(
                Some("Append"),
                Some("win.tab_append")
            );

        model.append_submenu(
            Some("Tab"),
            &model_tab
        );

        model.append(
            Some("Debug"),
            Some("win.debug")
        );

        model.append(
            Some("Quit"),
            Some("win.quit")
        );

    menu.set_menu_model(
        Some(&model)
    );

    menu
}