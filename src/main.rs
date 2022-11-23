use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    GamePlay,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    pub(crate) fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(11, "(Q) Quit");
    }
    pub(crate) fn play(&mut self, ctx: &mut BTerm) {
        // Todo finish this stub later
        self.mode = GameMode::End;
    }
    pub(crate) fn end(&mut self, ctx: &mut BTerm) {
        todo!()
    }

    pub(crate) fn restart(&mut self) {
        self.mode = GameMode::GamePlay
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::GamePlay => self.play(ctx),
            GameMode::End => self.end(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
