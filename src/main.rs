use std::path::PathBuf;

use err::ProjectorError;
use itertools::Itertools;
use projector::{ProjectionType, ProjectorParams};

mod err;
mod obj;
mod projector;

use crate::obj::ObjLoader;
use crate::projector::Projector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut obj = None;
    let mut output = None;
    let mut w = 512;
    let mut h = 512;
    let mut proj_type = ProjectionType::Face;
    for (k, v) in args.iter().skip(1).tuples() {
        if k == "--obj" {
            obj = Some(v);
        }
        if k == "--out" {
            output = Some(v);
        }
        if k == "--width" || k == "-w" {
            w = v.parse::<usize>()?;
        }
        if k == "--height" || k == "-h" {
            h = v.parse::<usize>()?;
        }

        if k == "--proj_type" || k == "-p" {
            proj_type = ProjectionType::from(v.as_str());
        }
    }

    let obj = if let Some(o) = obj {
        o
    } else {
        return Err(Box::new(ProjectorError::new("missing input .obj file")));
    };

    let obj_loader = ObjLoader::new(obj);
    let proj = Projector::new();

    let obj = obj_loader.load()?;
    let pixels = proj.project(
        &obj,
        ProjectorParams {
            width: w,
            height: h,
            kind: proj_type,
        },
    )?;

    let of = if let Some(of) = output {
        PathBuf::from(of)
    } else {
        let mut of = obj_loader.path().clone();
        of.set_extension("png");

        of
    };

    image::save_buffer::<PathBuf>(of, &pixels, w as u32, h as u32, image::ColorType::Rgb8)?;

    Ok(())
}
