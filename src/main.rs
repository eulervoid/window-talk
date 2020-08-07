use nannou::geom::rect::*;
use nannou::geom::Range;
use nannou::prelude::*;
use nannou::text::*;

mod pointer;
use pointer::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    windows: Vec<(Rect, String)>,
    pointer: Pointer,
    metro: Metronome,
    apercu: Font,
    words: std::iter::Cycle<std::slice::Iter<'static, &'static str>>,
}

struct Metronome {
    count: u64,
    every: u64,
}

impl Metronome {
    fn tick(&mut self) -> bool {
        self.count += 1;
        self.count % self.every == 0
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(60.0));
    // load font
    let font_data: &[u8] = include_bytes!("../duospace-regular.ttf");
    let apercu = Font::from_bytes(font_data).unwrap();

    Model {
        _window: app
            .new_window()
            .size(1280, 1280)
            .view(view)
            .build()
            .unwrap(),
        windows: Vec::new(),
        pointer: Pointer::Idle(Point2 { x: 50.0, y: 50.0 }),
        metro: Metronome {
            count: 0,
            every: 30,
        },
        apercu: apercu,
        words: [
            "hello",
            "there",
            "please",
            "follow",
            "me",
            "on",
            "instagram",
            "i",
            "need",
            "your",
            "clicks",
            "and",
            "likes",
            "and",
            "shares",
            "and",
            "actually",
            "some",
            "comments",
            "would",
            "also",
            "be",
            "wonderful",
            "thank",
            "you",
            "very",
            "much",
        ]
        .iter()
        .cycle(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let min_size = app.window_rect().h() * 0.25;
    let max_size = app.window_rect().h() * 0.90;
    if model.metro.tick() && random() {
        let frame = app.window_rect().pad(50.0);
        model.windows.push((
            random_rect(min_size, min_size, max_size, max_size, frame),
            String::from(model.words.next().unwrap().clone()),
        ));
    }

    if let Some((window, _text)) = model.windows.last() {
        model.pointer = model.pointer.update(Some(close_button_pos(window)));
    }

    if let (Pointer::Idle(pos), Some((window, _text))) = (model.pointer, model.windows.last()) {
        if (close_button_pos(window) - pos).magnitude() < 0.1 {
            model.windows.pop();
        }
    }
}

fn random_rect(min_w: f32, min_h: f32, max_w: f32, max_h: f32, contained_in: Rect) -> Rect {
    let half_w = random_range(min_w, max_w) / 2.0;
    let half_h = random_range(min_h, max_h) / 2.0;
    let center_container = contained_in.padding(Padding {
        x: Range::new(half_w, half_w),
        y: Range::new(half_h, half_h),
    });
    Rect::from_xy_wh(
        random_point(center_container),
        Vector2::from((half_w * 2.0, half_h * 2.0)),
    )
}

fn random_point(contained_in: Rect) -> Point2 {
    Point2 {
        x: random_range(contained_in.left(), contained_in.right()),
        y: random_range(contained_in.bottom(), contained_in.top()),
    }
}

fn close_button_pos(window: &Rect) -> Point2 {
    window.pad(20.0).top_left()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for (w, t) in model.windows.iter() {
        draw.rect()
            .xy(w.xy())
            .wh(w.wh())
            .stroke_weight(4.0)
            .stroke(WHITE)
            .color(BLACK);
        let tl = close_button_pos(w);
        for i in 0..3 {
            draw.ellipse()
                .x_y(tl.x + (20.0 * i as f32), tl.y)
                .w_h(14.0, 14.0)
                .color(BLUE);
        }
        draw.text(t)
            .color(WHITE)
            .font_size(48)
            .font(model.apercu.clone())
            .x_y(w.x(), w.y() + 10.0)
            .wh(w.wh());
    }

    // draw pointer
    let pointer_pos = model.pointer.position();
    draw.arrow()
        .weight(4.5)
        .points(pointer_pos + Vector2 { x: 15.0, y: -25.0 }, pointer_pos)
        .color(RED);
    // render frame
    draw.to_frame(app, &frame).unwrap();
    // capture frames to pngs
    //let file_path = captured_frame_path(app, &frame);
    //app.main_window().capture_frame(file_path);
}

fn _captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("frame-{:04}", frame.nth()))
        .with_extension("png")
}
