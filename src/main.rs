use bracket_lib::prelude::*;

use vim_quake::game;
use vim_quake::renderer;
use vim_quake::types::App;

struct AppWrapper {
    app: App,
}

impl GameState for AppWrapper {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.app.refresh_time();

        if let Some(key) = ctx.key {
            game::handle_key(&mut self.app, key, ctx.shift);
        }

        game::tick(&mut self.app, f64::from(ctx.frame_time_ms));

        self.app.update_visibility();

        renderer::render(ctx, &self.app);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("vim-quake")
        .build()?;
    main_loop(context, AppWrapper { app: App::new() })
}
