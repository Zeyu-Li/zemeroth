use macroquad::{
    self as mq,
    prelude::{RED, WHITE},
};
use zgui as ui;

mod common;

#[derive(Clone, Copy, Debug)]
enum Message {
    Command,
}

fn make_gui(font: mq::text::Font) -> ui::Result<ui::Gui<Message>> {
    let font_size = 64;
    let mut gui = ui::Gui::new();
    let anchor = ui::Anchor(ui::HAnchor::Right, ui::VAnchor::Bottom);
    let text = ui::Drawable::text("Button", font, font_size);
    let button = ui::Button::new(text, 0.2, gui.sender(), Message::Command)?;
    gui.add(&ui::pack(button), anchor);
    Ok(gui)
}

fn draw_scene() {
    let x = 0.0;
    let y = 0.0;
    let r = 0.4;
    let color = RED;
    mq::shapes::draw_circle(x, y, r, color);
}

#[macroquad::main("ZGui: Absolute Coordinates Demo")]
async fn main() {
    let assets = common::Assets::load().await;
    let mut gui = make_gui(assets.font).expect("TODO: err msg");
    loop {
        // Update the camera and the GUI.
        let aspect_ratio = common::aspect_ratio();
        let camera = common::make_and_set_camera(aspect_ratio);
        gui.resize(aspect_ratio);
        // Handle cursor updates.
        let pos = common::get_world_mouse_pos(&camera);
        gui.move_mouse(pos);
        if mq::input::is_mouse_button_pressed(mq::input::MouseButton::Left) {
            let message = gui.click(pos);
            println!("{:?}", message);
        }
        // Draw the GUI.
        mq::window::clear_background(WHITE);
        draw_scene();
        gui.draw();
        mq::window::next_frame().await;
    }
}
