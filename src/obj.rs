use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Div, DivAssign},
    path::PathBuf,
};

use crate::{
    err::{ProjectorError, ProjectorResult},
    proj_err,
};

#[derive(Debug, Clone, Copy)]
pub struct Vert {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vert {
    pub fn new() -> Vert {
        Vert {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}

impl AddAssign for Vert {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add<Vert> for Vert {
    type Output = Vert;

    fn add(self, rhs: Vert) -> Self::Output {
        Vert {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vert {
    type Output = Vert;

    fn add(self, rhs: f32) -> Self::Output {
        Vert {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Div<f32> for Vert {
    type Output = Vert;

    fn div(self, rhs: f32) -> Self::Output {
        Vert {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vert {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

pub type Face = [u32; 3];

pub struct Obj {
    pub vertices: Vec<Vert>,
    pub faces: Vec<Face>,
}

pub struct ObjLoader {
    path: PathBuf,
}

impl ObjLoader {
    pub fn new(filename: &str) -> ObjLoader {
        ObjLoader {
            path: PathBuf::from(filename),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn load(&self) -> ProjectorResult<Obj> {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let f = if let Ok(f) = File::open(&self.path) {
            f
        } else {
            return proj_err!(&format!(
                "Failed to open file {}",
                self.path.to_str().unwrap_or("<invalid_file>")
            ));
        };
        let r = BufReader::new(f);

        let mut max = u32::MIN;
        for line in r.lines() {
            let line = line?;
            let toks = line.split(' ').collect::<Vec<&str>>();
            if toks.is_empty() {
                return proj_err!("Invalid object file!");
            }
            match toks[0] {
                "v" => {
                    let v = Vert {
                        x: toks[1].parse::<f32>()?,
                        y: toks[2].parse::<f32>()?,
                        z: toks[3].parse::<f32>()?,
                    };
                    vertices.push(v);
                }
                "f" => {
                    let mut face = [0, 0, 0];
                    for i in 1..4 {
                        let toks = toks[i].split('/').collect::<Vec<&str>>();
                        face[i - 1] = toks[0].parse::<u32>()? - 1;
                        max = max.max(face[i - 1]);
                    }
                    faces.push(face);
                }
                _ => continue,
            }
        }

        println!("Vertices: {}, Max: {max}", vertices.len());

        Ok(Obj { vertices, faces })
    }
}
