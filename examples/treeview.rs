extern crate tuix;

use tuix::*;

use tuix::widgets::{Panel, ResizableVBox, ScrollContainer};

static THEME: &'static str = include_str!("themes/treeview_theme.css");

fn main() {
    //let event_loop = EventLoop::new();
    //Create the glutin window
    //let window = Window::new(&event_loop, WindowDescription::new().with_title("Panels").with_inner_size(800, 600));

    // Create the app
    let app = Application::new(|state, window| {
        state.add_theme(THEME);

        window.set_title("Panels").set_inner_size(800, 600);

        let rvbox = ResizableVBox::new().build(state, window.entity(), |context| {
            context
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::blue())
                .class("container")
        });

        let scroll = ScrollContainer::new().build(state, rvbox, |context| context);

        let root = Panel::new("ROOT").build(state, scroll, |context| context);

        let one = Panel::new("Level 1").build(state, root, |context| context.class("level1"));
        let _one_one = Label::new("Level 2").build(state, one, |context| context.class("level2"));
        let _one_two = Label::new("Level 2").build(state, one, |context| context.class("level2"));

        let one_three = Panel::new("Level 2").build(state, one, |context| context.class("level2"));
        let _one_three_one =
            Label::new("Level 3").build(state, one_three, |context| context.class("level3"));
        let _one_three_two =
            Label::new("Level 3").build(state, one_three, |context| context.class("level3"));
        let _one_four = Label::new("Level 2").build(state, one, |context| context.class("level2"));

        let two = Panel::new("Level 1").build(state, root, |context| context.class("level1"));
        let _two_one = Label::new("Level 2").build(state, two, |context| context.class("level2"));
        let _two_two = Label::new("Level 2").build(state, two, |context| context.class("level2"));
        let _two_three = Label::new("Level 2").build(state, two, |context| context.class("level2"));
        let _two_four = Label::new("Level 2").build(state, two, |context| context.class("level2"));

        let three = Panel::new("Level 1").build(state, root, |context| context.class("level1"));
        let three_one =
            Panel::new("Level 2").build(state, three, |context| context.class("level2"));
        let three_one_one =
            Panel::new("Level 3").build(state, three_one, |context| context.class("level3"));
        let _three_one_one_one =
            Label::new("Level 4").build(state, three_one_one, |context| context.class("level4"));
        let _three_one_one_two =
            Label::new("Level 4").build(state, three_one_one, |context| context.class("level4"));

    });

    app.run();
}
