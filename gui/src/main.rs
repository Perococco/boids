use raylib::prelude::*;

use gui::gui::Gui;


use gui::widget_operation::{WidgetOp, Size};

use gui::alignment::Alignment;
use gui::alignment::VAlignment::{Bottom, Top};
use gui::alignment::HAlignment::{Left, Right};

use gui::widget_data::WidgetDataProvider;
use gui::widget::Widget::Pane;
use gui::pane::PanePar;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut gui = Gui::new(|d| {
        let mut par = PanePar::new(d);
        par.set_fill_height(true)
            .set_requested_width(200.0)
            .set_position(0.0,0.0);
        Pane(par)
    });

    let font_id = gui.load_font(&mut rl, &thread,
            "/home/perococco/Downloads/FreckleFace-Regular.ttf",
            48,
            200,
        ).unwrap();

    let mut screen_size:Size = Size{width:rl.get_screen_width() as f32, height:rl.get_screen_height() as f32};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_window_resized() {
            screen_size = Size{width:d.get_screen_width() as f32, height:d.get_screen_height() as f32};
        }

        gui.layout(&screen_size);



        d.clear_background(Color::WHITE);

        gui.render(&mut d);

        d.draw_line(0, 120, screen_size.width as i32, 120, Color::RED);
        d.draw_line(120, 0, 120, screen_size.height as i32, Color::RED);
    }
}
