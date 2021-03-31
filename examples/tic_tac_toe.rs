use tuix::*;

fn calculate_winner(squares: &[GameData; 9]) -> GameData {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for i in 0..LINES.len() {
        let [a, b, c] = LINES[i];
        if squares[a] != GameData::Null && squares[a] == squares[b] && squares[b] == squares[c] {
            return squares[a];
        }
    }

    return GameData::Null;
}

// Data to describe the state of a square as well as the current player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameData {
    X,
    O,
    Null,
}

// The game events
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    SquarePressed(usize),
    ProcessOutcome(GameData),
    Restart,
}

// Widget to describe the board state
pub struct Board {
    squares: [GameData; 9],
    current_player: GameData,
    num_of_moves: usize,

    overlay: Entity,
    winner_label: Entity,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [GameData::Null; 9],
            current_player: GameData::O,
            num_of_moves: 0,

            overlay: Entity::default(),
            winner_label: Entity::default(),
        }
    }
}

// Add the squares and the post-game overlay
impl Widget for Board {
    type Ret = Entity;
    fn on_build(&mut self, mut context: Context) -> Self::Ret {

        let entity = context.entity();

        // Create three rows each with 3 buttons
        for r in 0..3 {
            let mut row = HBox::new().build(&mut context).class("row");
            for c in 0..3 {
                Square::default()
                    .on_press(Event::new(GameEvent::SquarePressed(3 * r + c)))
                    .build(&mut row).class("square");
            }
        }

        let mut overlay= Element::new().build(&mut context).class("overlay");
        self.overlay = overlay.entity();

        self.winner_label =
            Label::new("").build(&mut overlay).class("winner").entity();

        Button::with_label("Play Again")
            .on_release(Event::new(GameEvent::Restart).target(entity))
            .build(&mut overlay)
            .class("replay");

        context.set_element("board").entity()
    }

    // React to the various game events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(game_event) = event.message.downcast::<GameEvent>() {
            match game_event {
                GameEvent::SquarePressed(index) => {
                    match self.current_player {
                        GameData::O => {
                            event.origin.set_text(state, "O").set_disabled(state, true);
                            self.squares[*index] = GameData::O;
                            self.current_player = GameData::X;
                        }

                        GameData::X => {
                            event.origin.set_text(state, "X").set_disabled(state, true);
                            self.squares[*index] = GameData::X;
                            self.current_player = GameData::O;
                        }

                        _ => {}
                    }

                    self.num_of_moves += 1;

                    state.insert_event(
                        Event::new(GameEvent::ProcessOutcome(calculate_winner(&self.squares)))
                            .target(entity),
                    );

                    event.consume();
                }

                GameEvent::ProcessOutcome(player) => match player {
                    GameData::O => {
                        self.winner_label.set_text(state, "O's WIN!");
                        self.overlay.set_checked(state, true);
                    }

                    GameData::X => {
                        self.winner_label.set_text(state, "X's WIN!");
                        self.overlay.set_checked(state, true);
                    }

                    GameData::Null => {
                        if self.num_of_moves == 9 {
                            self.winner_label.set_text(state, "DRAW!");
                            self.overlay.set_checked(state, true);
                        }
                    }
                },

                GameEvent::Restart => {
                    self.overlay.set_checked(state, false);
                    self.squares = [GameData::Null; 9];
                    self.num_of_moves = 0;
                    state.insert_event(
                        Event::new(GameEvent::Restart)
                            .target(entity)
                            .propagate(Propagation::Fall),
                    );
                }
            }
        }
    }
}

// Widget to describe a square in the board
#[derive(Default)]
pub struct Square {
    button: Button,
}

impl Square {
    pub fn on_press(mut self, event: Event) -> Self {
        self.button = self.button.on_press(event);

        self
    }
}

// Inherits from button
impl Widget for Square {
    type Ret = Entity;
    fn on_build(&mut self, mut context: Context) -> Self::Ret {
        self.button.on_build(context.clone())
    }

    // Inherits button behaviour and adds new behaviour by reacting to a restart event
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);

        if let Some(game_event) = event.message.downcast::<GameEvent>() {
            match game_event {
                GameEvent::Restart => {
                    entity.set_text(state, "").set_disabled(state, false);
                }

                _ => {}
            }
        }
    }
}
// Run the app
fn main() {
    let app = Application::new(|mut context, window| {
        context.state()
            .add_stylesheet("examples/themes/tic_tac_toe_theme.css")
            .expect("Failed to load stylesheet");

        window.set_inner_size(300, 300).set_title("Tic Tac Toe");

        Board::new().build(&mut context);

        
    });

    app.run();
}
