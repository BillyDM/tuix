extern crate tuix;

use tuix::*;

use tuix::widgets::Button;

static THEME: &'static str = include_str!("themes/menus_theme.css");

// #[derive(Clone, Debug)]
// pub enum MenuBarEvent {
//     SetText(String),
//     OptionChanged(u32),
// }

// impl Message for MenuBarEvent {}

// pub struct MenuBar {
//     open_menu: Entity,
// }

// impl MenuBar {
//     pub fn new(text: &str) -> Self {
//         MenuBar {
//             open_menu: Entity::null(),
//         }
//     }
// }

// impl EventHandler for MenuBar {
//     fn build<'a>(
//         mut self,
//         state: &'a mut State,
//         parent: Entity,
//         event_manager: &'a mut EventManager,
//     ) -> Context<'a> {
//         let id = state.add(parent);
//         id.set_width(state, 200.0)
//             .set_height(state, 30.0)
//             .set_display(state, Display::Flexbox);

//         event_manager.build(id, parent, state, self)
//     }

//     fn handle_event(
//         &mut self,
//         id: Entity,
//         state: &mut State,
//         event: &Event,
//         event_manager: &mut EventManager,
//     ) -> bool {
//         // if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
//         //     match menu_event {

//         //     }
//         // }

//         if let Some(window_event) = event.message.downcast::<WindowEvent>() {
//             match window_event {
//                 WindowEvent::MouseMove(x, y) => {
//                     for child in id.child_iter(&state.hierarchy) {
//                         if child == state.hovered {
//                             //event_manager.insert_event(Event::new(MenuEvent::Open(child)).target(child).propagate(false));

//                             return false;
//                         }
//                     }
//                 }

//                 WindowEvent::MouseDown(button, mods) => match button {
//                     MouseButton::Left => {
//                         if state.hovered == id {
//                             event_manager.insert_event(Event::new(StyleEvent::Restyle);
//                         } else {

//                         }
//                     }
//                     _ => {}
//                 },

//                 WindowEvent::MouseUp(button, mods) => match button {
//                     MouseButton::Left => {}

//                     _ => {}
//                 },

//                 _ => {}
//             }
//         }

//         false
//     }
// }

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        state.add_theme(THEME);

        window.set_title("Menus");

        let menu_bar = MenuBar::new().build(state, window.entity(), |context| {
            context.set_flex_direction(FlexDirection::Row)
        });

        let menu1 = Menu::new().build(state, menu_bar, |context| {
            context
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_flex_grow(0.0)
                .set_text_justify(Justify::Center)
                .class("menu")
        });

        Button::with_label("Item 1").build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 2").build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 3")
            .on_press(Event::new(WindowEvent::WindowClose))
            .build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 4").build(state, menu1, |context| context.class("item"));

        let menu1 = Menu::new().build(state, menu_bar, |context| {
            context
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_flex_grow(0.0)
                .set_text_justify(Justify::Center)
                .class("menu")
        });

        Button::with_label("Item 1").build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 2").build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 3")
            .on_press(Event::new(WindowEvent::WindowClose))
            .build(state, menu1, |context| context.class("item"));
        Button::with_label("Item 4").build(state, menu1, |context| context.class("item"));

        // // Button::new().build2(state, menu1, |context| context.class("spacer2"));

        // Button::with_label("SubItem 1").build(state, menu2, |context| context.class("item"));
        // Button::with_label("SubItem 2").build(state, menu2, |context| context.class("item"));
        // Button::with_label("SubItem 3")
        //     .on_press(Event::new(WindowEvent::WindowClose))
        //     .build(state, menu2, |context| context.class("item"));

        // Button::new().build(state, menu1, |context| context.class("spacer2"));

        // let menu3 = Menu::new("SubSubMenu", MenuPosition::Right).build(state, menu2, |context| {
        //     context.class("item").class("submenu")
        // });

        // Button::with_label("SubSubItem 1").build(state, menu3, |context| context.class("item"));
        // Button::with_label("SubSubItem 2").build(state, menu3, |context| context.class("item"));
        // Button::with_label("SubSubItem 3")
        //     .on_press(Event::new(WindowEvent::WindowClose))
        //     .build(state, menu3, |context| context.class("item"));

        // let button = Button::with_label("Right Click Me").build(state, window, |context| {
        //     context
        //         .set_left(Length::Pixels(100.0))
        //         .set_top(Length::Pixels(100.0))
        //         .set_width(Length::Pixels(150.0))
        //         .set_height(Length::Pixels(30.0))
        //         .set_background_color(Color::green())
        // });

        // let (_container, menu) = ContextMenu::new().build(state, button, |context| {
        //     context
        //         .set_width(Length::Percentage(1.0))
        //         .set_height(Length::Percentage(1.0))
        // });

        // menu.set_width(state, Length::Pixels(100.0));

        // Button::with_label("Option 1").build(state, menu, |context| {
        //     context
        //         .set_height(Length::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });

        // Button::with_label("Option 2").build(state, menu, |context| {
        //     context
        //         .set_height(Length::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });

        // Button::with_label("Option 3").build(state, menu, |context| {
        //     context
        //         .set_height(Length::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });

        
    });

    app.run();
}
