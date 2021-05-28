use tuix::*;
use rand::prelude::*;

const STYLE: &str = r#"

    button {
        child-space: 1s;
        border-radius: 3px;
        color: black;
        background-color: #d2d2d2;
    }

    button:hover {
        background-color: #e2e2e2;
    }

    button:active {
        background-color: #c2c2c2;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    button: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.button = Button::with_label("Change Color")
            .on_press(|button, state, entity| {
                let r: u8 = rand::thread_rng().gen();
                let g: u8 = rand::thread_rng().gen();
                let b: u8 = rand::thread_rng().gen();
                state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(r, g, b, 255))).target(entity));
            })
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        entity.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    entity.set_background_color(state, *color);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Button")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}