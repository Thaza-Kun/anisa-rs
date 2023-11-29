#![allow(unused_imports)]

use bodies::AstroBody;
use clap::Parser;
use nannou::prelude::*;
use quantities::dynamics::Force;
use quantities::spatial::{Cartesian, Velocity};
use quantities::Tensor;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use nalgebra::SimdComplexField;
use units::length::meter::Meter;

mod bodies;
mod quantities;
mod scene;
mod units;

#[derive(Debug)]
struct Consts {
    grav: f32,
    step_time: usize,
    lock_at: usize,
}

#[derive(serde::Deserialize)]
struct Planet {
    pub mass: f32,
    pub velocity: f32,
    pub distance: f32,
    pub color: String,
}

#[derive(serde::Deserialize)]
struct Config {
    pub planets: Vec<Planet>,
}

struct GravConst;

// TODO Fix scaling

impl GravConst {
    const MASS_EARTH: f32 = 5.9722E24;
    const DIST_AU: f32 = 149.6e6 * 1000.; //m per au
    const G: f32 = 6.67428e-11; // N m2 per kg2
    const SCALE: f32= 250. / Self::DIST_AU; // 1AU = 100 pixel
    const TIME_STEP: f32 = 3600.*24.; // 1 day
}

impl Planet {
    fn to_body_relative_to(
        &self,
        host: &AstroBody<Cartesian<f32, 2, Meter>, 2, 1>,
        angle: &f32
    ) -> AstroBody<Cartesian<f32, 2, Meter>, 2, 1> {
        let here = Cartesian::with_magnitude(self.distance, *angle, host.pos);
        AstroBody::new_dynamic(
            self.mass,
            here,
            Velocity::new_perpendicular_to(
                self.velocity * 0.66e-3 * 100., // pixel per seconds
                &here,
                &host.pos,
                0.,
            ),
        ).set_color(self.color.clone())
    }
}

#[derive(clap::Parser)]
struct Args {
    #[arg(long, short)]
    file: String,
    #[arg(short)]
    grav: f32,
    #[arg(short, long)]
    lock: usize,
    #[arg(short, long)]
    speed: Option<usize>,
}

fn read_config(file: PathBuf) -> Result<Config, nannou::io::TomlFileLoadError> {
    load_from_toml(file)
}

fn main() {
    nannou::app(scene::setup)
        .update(scene::update)
        .simple_window(scene::view)
        .run();
}
