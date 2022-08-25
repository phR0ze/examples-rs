use macroquad::prelude::*;
use macroquad_tantan_toolbox::animation::*;

#[derive(std::hash::Hash, Eq, PartialEq)]
enum CatAnimationIdentifier {
    Run,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pakémon".to_owned(),
        window_width: 320,
        window_height: 240,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_title: Texture2D = load_texture("assets/title.png").await.unwrap();
    let texture_far: Texture2D = load_texture("assets/far-buildings.png").await.unwrap();
    let texture_back: Texture2D = load_texture("assets/back-buildings.png").await.unwrap();
    let texture_foreground: Texture2D = load_texture("assets/foreground.png").await.unwrap();
    let texture_cat: Texture2D = load_texture("assets/cat.png").await.unwrap();

    let mut position_title = 0. - texture_title.height();
    let mut position_far = texture_far.width();
    let mut position_back = texture_back.width();
    let mut position_foreground = texture_foreground.width();

    let mut animation = AnimationInstance::<CatAnimationIdentifier>::new(
        6.,
        12.,
        texture_cat,
        CatAnimationIdentifier::Run,
    );
    animation.add_animation(36, 39, None, 4., CatAnimationIdentifier::Run);
    animation.play_animation(CatAnimationIdentifier::Run);

    loop {
        let time = get_frame_time();

        clear_background(BLACK);

        position_far -= 0.05;
        if position_far < 0. {
            position_far = texture_far.width();
        }

        draw_texture(texture_far, position_far, 0., WHITE);
        draw_texture(texture_far, position_far - texture_far.width(), 0., WHITE);

        position_back -= 0.1;
        if position_back < 0. {
            position_back = texture_back.width();
        }

        draw_texture(texture_back, position_back, -30., WHITE);
        draw_texture(
            texture_back,
            position_back - texture_back.width(),
            -30.,
            WHITE,
        );

        position_foreground -= 0.5;
        if position_foreground < 0. {
            position_foreground = texture_foreground.width();
        }

        draw_texture(texture_foreground, position_foreground, 50., WHITE);
        draw_texture(
            texture_foreground,
            position_foreground - texture_foreground.width(),
            50.,
            WHITE,
        );

        if position_title <= 30. {
            position_title += 0.5;
        } else {
            draw_text("PRESS START", 124.0, 120.0, 16.0, WHITE);
        }

        draw_texture(
            texture_title,
            160. - texture_title.width() / 2.,
            position_title,
            WHITE,
        );

        animation.update(time);
        animation.draw(&vec2(160., 210.), false);

        next_frame().await
    }
}
