use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Div, DivAssign, Neg, Sub},
    path::PathBuf,
};

use crate::{
    err::{ProjectorError, ProjectorResult},
    proj_err,
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn from_vals(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from_val(val: f32) -> Vec3 {
        Vec3 {
            x: val,
            y: val,
            z: val,
        }
    }

    pub fn set_elem(&mut self, idx: u32, val: f32) {
        match idx {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => (),
        }
    }

    pub fn get(&self, n: usize) -> f32 {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => f32::MAX,
        }
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.x * v.z - self.z * v.x,
            z: self.x * v.y - self.y * v.z,
        }
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

pub struct Face {
    pub v: [usize; 3],
    pub n: [usize; 3],
}

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
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
        let mut normals = Vec::new();
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

        for line in r.lines() {
            let line = line?;
            let toks = line.split(' ').collect::<Vec<&str>>();
            if toks.is_empty() {
                return proj_err!("Invalid object file!");
            }
            match toks[0] {
                "v" => {
                    let v = Vec3 {
                        x: toks[1].parse::<f32>()?,
                        y: toks[2].parse::<f32>()?,
                        z: toks[3].parse::<f32>()?,
                    };
                    vertices.push(v);
                }
                "vn" => {
                    let v = Vec3 {
                        x: toks[1].parse::<f32>()?,
                        y: toks[2].parse::<f32>()?,
                        z: toks[3].parse::<f32>()?,
                    };
                    normals.push(v);
                }
                "f" => {
                    let mut face = Face {
                        v: [0; 3],
                        n: [0; 3],
                    };
                    for i in 1..4 {
                        let toks = toks[i].split('/').collect::<Vec<&str>>();
                        face.v[i - 1] = toks[0].parse::<usize>()? - 1;
                        face.n[i - 1] = toks[2].parse::<usize>()? - 1;
                    }
                    faces.push(face);
                }
                _ => continue,
            }
        }

        Ok(Obj {
            vertices,
            normals,
            faces,
        })
    }
}
