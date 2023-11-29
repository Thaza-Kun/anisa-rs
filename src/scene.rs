use crate::bodies::{AstroBody, Tracer};
use crate::quantities::spatial::Cartesian;
use crate::{Args, Consts, GravConst, read_config};
use clap::Parser;
use nannou::color::{BLACK, BLUE, RED};
use nannou::event::Update;
use nannou::{App, Frame};
use std::path::PathBuf;
use crate::units::length::pixel::Pixel;
use crate::units::length::meter::Meter;

#[derive(Debug)]
pub struct Scene {
    bodies: Vec<AstroBody<Cartesian<f32, 2, Meter>, 2, 1>>,
    tracers: Vec<Tracer<Cartesian<f32, 2, Pixel>, 2, 1>>,
    consts: Consts,
}

pub fn setup(_app: &App) -> Scene {
    let args = Args::parse();

    let config = read_config(PathBuf::from(&args.file)).expect("Error reading file");
    let planets = config.planets;


    let (first, others) = planets.split_first().unwrap().into();
    let angles = ndarray::Array::linspace(0., 360., others.len());
    let host: AstroBody<Cartesian<f32, 2, Meter>, 2, 1> = AstroBody::from(first);


    let mut bodies = Vec::<AstroBody<Cartesian<f32, 2, Meter>, 2, 1>>::new();
    let mut tracers = Vec::<Tracer<Cartesian<f32, 2, Pixel>, 2, 1>>::new();
    bodies.push(host.clone());
    tracers.push(Tracer::default());

    for (planet, angle) in others.iter().zip(angles.iter()) {
        bodies.push(
            planet.to_body_relative_to(&host, angle)
        );
        tracers.push(Tracer::default())
    }
    dbg!(Scene {
        bodies,
        tracers,
        consts: Consts {
            grav: GravConst::G,
            lock_at: args.lock,
            step_time: match args.speed {
                Some(s) => s,
                None => 1,
            },
        },
    })
}

pub fn update(_app: &App, _model: &mut Scene, _update: Update) {
    let bodies = _model.bodies.clone();
    let shift = bodies[_model.consts.lock_at].get_shift_from_origin();
    let vel = bodies[_model.consts.lock_at].get_velocity_shift_from_origin();
        for (i, (body, trace)) in _model
            .bodies
            .iter_mut()
            .zip(_model.tracers.iter_mut())
            .enumerate()
        {
            body.gravitate(&bodies, &_model.consts.grav);
            if i == _model.consts.lock_at {
                body.shift_by(&shift, &vel);
            } else {
                body.shift_by(&-shift, &-vel);
                body.update(1.);
            }
            trace.pos.push_front(Cartesian::from(body.pos.clone()));
            // trace.pos.resize(1000, Cartesian::from(body.pos.clone()));
        }
}

pub fn view(_app: &App, _model: &Scene, _frame: Frame) {
    let draw = _app.draw();

    draw.background().color(BLACK);

    for (body, trace) in _model.bodies.iter().zip(_model.tracers.iter()) {
        let coords = Cartesian::<f32, 2, Pixel>::from(body.pos);
        // TODO Impl color
        draw.ellipse()
            .w_h(body.radius, body.radius)
            .x_y(coords.horizontal(), coords.vertical())
            .color(BLUE);
        for t in trace.pos.iter() {
            draw.ellipse()
                .w_h(1., 1.)
                .color(RED)
                .x_y(t.horizontal(), t.vertical());
        }
    }
    draw.to_frame(_app, &_frame).unwrap();
}
