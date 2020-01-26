use rltk::{Rltk, GameState, Console};
struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Houston");
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "TARS", "resources");
    let gs =  State{};
    rltk::main_loop(context, gs);
}