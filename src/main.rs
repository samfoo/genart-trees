extern crate rand;
extern crate cairo;
extern crate ndarray;
extern crate ndarray_rand;

pub mod tree;

use std::io::BufWriter;
use std::fs::File;
use std::io::prelude::*;
use std::f64::consts::PI;
use rand::Rng;
use rand::distributions::Range;
use ndarray::{Array1, Dim};
use ndarray_rand::RandomExt;
use std::path::Path;

// const SIZE: f64 = 10000.0;
const SIZE: f64 = 5000.0;

fn render_branch(ctx: &mut cairo::Context, b: &tree::Branch) {
    let rng = &mut rand::thread_rng();

    let x = b.x;
    let y = b.y;
    let a = b.angle;
    let r = b.radius;

    if r < 0.0 {
        return;
    }

    let x1 = x + (a-0.5*PI).cos()*r;
    let x2 = x + (a+0.5*PI).cos()*r;
    let y1 = y + (a-0.5*PI).sin()*r;
    let y2 = y + (a+0.5*PI).sin()*r;

    // println!("{}, {}", x1, y1);
    // println!("{}, {}", x2, y2);

    // trunk
    ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
    ctx.set_line_width(1.0 / SIZE / 2.0);
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke();

    // outline
    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.45);
    ctx.rectangle(x1, y1, 1.0 / SIZE * 6.0, 1.0 / SIZE * 6.0);
    ctx.fill();

    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.45);
    ctx.rectangle(x2, y2, 1.0 / SIZE * 4.0, 1.0 / SIZE * 4.0);
    ctx.fill();

    // bark texture
    let dd = ((x-x2).powf(2.0) + (y-y2).powf(2.0)).sqrt();
    let the = 0.5 * PI + a;

    let dim = Dim([(rng.next_u32() % 5) as usize]);
    let shadow_right = Array1::random(dim, Range::new(0.0, 1.0)) * dd * rng.next_f64();
    let xxp = x2 - &shadow_right * the.cos();
    let yyp = y2 - &shadow_right * the.sin();


    ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    for i in 0..shadow_right.len() {
        let x_1 = *xxp.get(i).unwrap();
        let y_1 = *yyp.get(i).unwrap();

        ctx.rectangle(x_1, y_1, 1.0/SIZE*6.0, 1.0/SIZE*6.0);
        ctx.fill();
    }

    let dim = Dim([(rng.next_u32() % 3) as usize]);
    let shadow_left = Array1::random(dim, Range::new(0.0, 1.0)) * dd * rng.next_f64();
    let xxp = x1 + &shadow_left * the.cos();
    let yyp = y1 + &shadow_left * the.sin();

    for i in 0..shadow_left.len() {
        let x_1 = *xxp.get(i).unwrap();
        let y_1 = *yyp.get(i).unwrap();

        ctx.rectangle(x_1, y_1, 1.0/SIZE*4.0, 1.0/SIZE*4.0);
        ctx.fill();
    }
}

fn output() -> String {
    if Path::new("./out.png").exists() {
        for i in 0..1000 {
            let s = format!("./out-{}.png", i);
            let p = Path::new(&s);

            if !p.exists() {
                return s.clone();
            }
        }

        panic!("can't file suitable output file");
    } else {
        String::from("./out.png")
    }
}

fn main() {
    let mut t = tree::Tree::new(
        0.5,
        0.95,
        // 0.02,
        100.0 / SIZE,
        -PI*0.5,
        1.0 / SIZE / 40.0,
        SIZE
    );

    let mut surface = cairo::ImageSurface::create(
        cairo::Format::ARgb32,
        SIZE as i32, SIZE as i32
    );

    let mut ctx = cairo::Context::new(&mut surface);

    ctx.scale(SIZE, SIZE);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.rectangle(0.0, 0.0, 1.0, 1.0);
    ctx.fill();

    while t.branches.len() > 0 {
        t.step();

        for ref b in t.branches.iter() {
            render_branch(&mut ctx, b);
        }
    }

    let mut output = File::create(output()).unwrap();
    let mut f = BufWriter::new(output);

    surface.write_to_png(&mut f);
}
