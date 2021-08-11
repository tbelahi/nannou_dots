use nannou::{color::named, prelude::*};
use palette::Srgb;

fn main() {
    nannou::app(model).update(update).run();
}

struct Dot {
    x: f32,
    y: f32,
    radius: f32,
    offset: f32,
    color: Srgb,
}

struct Model {
    bg_color: String,
    dots: Vec<Dot>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let n_dots = 100;
    let mut dots: Vec<Dot> = Vec::new();
    for _ in 1..=n_dots {
        let dot = Dot {
            x: random_range(-50.0, 50.0), //useless as it is overwritten as soon as loop starts
            y: random_range(-10.0, 10.0),
            radius: 10.0,
            offset: random_range(-50.0, 50.0),
            color: Srgb::new(
                random_range(0.0, 1.0),
                random_range(0.0, 1.0),
                random_range(0.0, 1.0),
            ),
        };
        dots.push(dot);
    }

    Model {
        bg_color: "honeydew".to_string(),
        dots: dots,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for mut dot in &mut model.dots {
        let t = app.time - dot.offset;
        let sine = t.sin();
        let cosine = -t.sin();
        let slowersine = (t / 2.0_f32).sin();
        let boundary = app.window_rect();
        // dot.x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
        // dot.y = map_range(cosine, -1.0, 1.0, boundary.bottom(), boundary.top());
        dot.x = dot.x + sine;
        dot.y = dot.y + cosine;

        /*     if model.radius < 500.0 {
            model.radius += 2.0;
        } */

        let fastersine = (t * 2.0).sin();
        dot.radius = map_range(fastersine, -1.0, 1.0, 10.0, 100.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let bg_color = named::from_str(&model.bg_color).expect("unknown color");

    draw.background().color(bg_color);
    for dot in &model.dots {
        draw.ellipse()
            .color(dot.color)
            .w(dot.radius)
            .h(dot.radius)
            .x_y(dot.x, dot.y);
    }
    draw.to_frame(app, &frame).unwrap();
}
