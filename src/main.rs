use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike tutorial")
        .build()
        .expect("Context errored out");

    let gs = State {};
    rltk::main_loop(context, gs).expect("Main loop errored out")
}
