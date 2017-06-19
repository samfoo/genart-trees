extern crate rand;

use rand::distributions::{Normal, IndependentSample};
use rand::Rng;
use std::f64::consts::PI;

pub struct Branch {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub angle: f64,
    pub iteration: i64,
    pub diminish: f64,
    pub size: f64,
}

impl Branch {
    pub fn step(&mut self) {
        let n = Normal::new(0.0, 1.0);

        // reduce radius by some amount
        self.radius -= self.diminish;

        // alter the angle by some amount
        let angle = n.ind_sample(&mut rand::thread_rng()) * 12.0 * PI / 1000.0;
        // let da = 2.2 / (self.radius * 500.0);
        let scale = (1.0 / self.size) + 0.01 - self.radius;
        let da = (1.0 + scale / 0.01).powf(1.2);
        self.angle += da * angle;

        // change the x/y
        let dx = (1.0 / self.size) * self.angle.cos();
        let dy = (1.0 / self.size) * self.angle.sin() * 1.3;

        // println!("dx: {}, dy: {}", dx, dy);
        // println!("x: {}, y: {}", self.x, self.y);

        self.x += dx;
        self.y += dy;
        // println!("x': {}, y': {}", self.x, self.y);

        self.iteration += 1;
    }
}

pub struct Tree {
    pub init_x: f64,
    pub init_y: f64,
    pub init_radius: f64,
    pub init_angle: f64,
    pub iteration: i64,
    pub diminish: f64,
    pub branches: Vec<Branch>,
    pub size: f64,
}

impl Tree {
    pub fn new(x: f64, y: f64, r: f64, a: f64, d: f64, s: f64) -> Tree {
        let root = Branch {
            x: x,
            y: y,
            angle: a,
            radius: r,
            iteration: 0,
            diminish: d,
            size: s
        };

        let mut bs: Vec<Branch> = Vec::new();
        bs.push(root);

        return Tree { init_x: x, init_y: y, init_radius: r, init_angle: a, iteration: 0, diminish: d, branches: bs, size: s }
    }

    pub fn step(&mut self) {
        let mut new_branches: Vec<Branch> = Vec::new();

        for b in self.branches.iter_mut() {
            b.step();

            if b.radius < (self.init_radius / 100.0) {
                // println!("b.radius: {}", b.radius);
                continue;
            } else {
                // determine if spawning another branch
                let branch_prob = (self.init_radius - b.radius) * 0.15;

                println!("branch prob: {}", branch_prob);

                let rng = &mut rand::thread_rng();

                if rng.next_f64() < branch_prob {
                    let x = b.x;
                    let y = b.y;
                    let r = 0.71 * b.radius;
                    let l_or_r: f64 = (-1 as i32).pow(rng.next_u32() % 2) as f64;
                    let ra = 0.3 * PI * rng.next_f64() * l_or_r;
                    let a = b.angle + l_or_r;

                    new_branches.push(Branch {
                        x: x, y: y, angle: a, radius: r, iteration: 0, diminish: self.diminish, size: self.size
                    });
                }
            }
        }

        self.branches.retain(|ref b| b.radius > 0.0);
        self.branches.extend(new_branches);
    }
}
