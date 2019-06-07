mod delaun;
mod draw;

use delaun::{Coord, Delaun};
use draw::App;

fn main() -> std::result::Result<(), String> {
    // let app = App::new();
    // app.run_app()?;

    let d = Delaun::calc(vec![
        Coord::new(3.0, 4.0),
        Coord::new(5.0, 2.0),
        Coord::new(4.0, 6.0),
    ]);

    Ok(())
}
