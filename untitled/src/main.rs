use std::f64::consts::PI;
use std::ops::{Range, Deref};
use std::time::{Instant, Duration};
use cairo::Context;
use gtk::{Inhibit, ContainerExt, WidgetExt, BoxExt, Button, LabelExt};
use gtk::prelude::{BuilderExtManual, WidgetExtManual};
use std::sync::{Arc, Mutex};

fn main() {
    gtk::init();
    let builder = gtk::Builder::from_file("untitled.glade");
    let window: gtk::Window = builder.get_object("main_window").unwrap();

    let label: gtk::Label = builder.get_object("label").unwrap();

    let chunks = Arc::new(Mutex::new(0u32));
    show(&builder, *chunks.lock().unwrap());

    let draw2: gtk::DrawingArea = builder.get_object("draw2").unwrap();

    let points = Arc::new(Mutex::new(Vec::<f64>::new()));
    let points_cloned = points.clone();
    draw2.set_size_request(500, 500);
    draw2.connect_draw(move | _, context| {
        context.move_to(0.0, 250.0);
        context.set_source_rgb(0.8, 0.0, 0.0);
        context.line_to(500.0, 250.0);
        context.stroke();

        context.set_source_rgb(0.0, 0.0, 0.0);
        context.move_to(0.0, 0.0);
        let mut x = 0;
        for y in points.lock().unwrap().deref() {
            let a = ((PI * 5000.0) - (*y * 5000.0)) + 250.0;
            context.line_to(x as f64, a);
            x += 5;
        }
        context.stroke();
        Inhibit(false)
    });

    let button: gtk::Button = builder.get_object("button").unwrap();

    let chunks_cloned = chunks.clone();
    let builder_cloned = builder.clone();

    button.connect_button_press_event(move | button, _ | {
        *chunks_cloned.lock().unwrap() += 5;
        show(&builder_cloned, *chunks_cloned.lock().unwrap());

        let radius = 500f64; //원의 반지름

        let chunks = *chunks_cloned.lock().unwrap(); //쪼개는 갯수

        let chunk_size = radius / chunks as f64; //도형 하나의 너비

        let mut sum = 0f64; // upper sum 과 lower sum의 합을 저장할 변수

        for chunk/* n */ in 0..chunks { //반복문 0부터 chunks번까지
            let y = (500u32.pow(2) as f64 - (chunk as f64 * radius / chunks as f64).powi(2)).sqrt(); // 반지름제곱 빼기 너비제곱
            sum += if chunk == 0 {
                y * chunk_size // 첫번째 도형일 경우 y * 너비를 총합에 더하기 -> upper sum
            } else {
                y * 2f64 * chunk_size// 첫번째 도형이 아닐 경우 upper sum 을 두번 더하기(*2)
            };
        }

        let size = sum * 2f64; //원넓이 = (upper sum + lower sum) / 2 * 4 = (upper sum + lower sum) * 2
        let pi = size / radius.powi(2); //원주율 = 원넓이 / 반지름제곱

        label.set_text(&pi.to_string());

        points_cloned.lock().unwrap().push(pi);


        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn show(builder: &gtk::Builder, chunks: u32) {
    let main_box: gtk::Box = builder.get_object("main_box").unwrap();
    let drawing_area: gtk::DrawingArea = gtk::DrawingArea::new();

    drawing_area.set_size_request(500, 500);
    drawing_area.connect_draw(move |da, context| {
        draw(context, chunks);
        Inhibit(true)
    });
    let r = &main_box.get_children()[0];
    main_box.remove(r);
    main_box.add(&drawing_area);
    main_box.set_child_position(&drawing_area, 0);
    drawing_area.show();
}

fn draw(context: &Context, chunks: u32) {
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.translate(0.0, 0.0);
    let chunk_size = 500f64 / chunks as f64;
    for i in (0..chunks).rev() {
        let x = i as f64 * 500 as f64 / chunks as f64;
        let y = (500u32.pow(2) as f64 - x.powi(2)).sqrt();
        context.set_source_rgb(51f64 / 255f64, 204f64 / 255f64, 255f64 / 255f64);
        context.rectangle(x, 500f64 - y, chunk_size, y);
        context.stroke();
        if i > 0 {
            context.set_source_rgb(0f64, 102f64 / 255f64, 255f64 / 255f64);
            context.rectangle(x, 500f64 - y, -chunk_size, y);
            context.fill();
        }
    }
    context.reset_clip();
}