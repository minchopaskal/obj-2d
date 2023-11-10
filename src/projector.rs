use crate::{
    err::ProjectorResult,
    obj::{Obj, Vec3},
};

#[derive(PartialEq)]
pub enum ProjectionType {
    Face,
    Vertex,
    VertexNormal,
    FaceNormal,
}

impl From<&str> for ProjectionType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "face" | "f" => ProjectionType::Face,
            "vertex" | "v" => ProjectionType::Vertex,
            "vertex_normal" | "vn" => ProjectionType::VertexNormal,
            "face_normal" | "fn" => ProjectionType::FaceNormal,
            _ => panic!("Unknown projection type {value}"),
        }
    }
}

pub struct ProjectorParams {
    pub width: usize,
    pub height: usize,
    pub kind: ProjectionType,
}

pub struct Projector {}

impl Projector {
    pub fn new() -> Projector {
        Projector {}
    }

    pub fn project(&self, obj: &Obj, params: ProjectorParams) -> ProjectorResult<Vec<u8>> {
        let w = params.width;
        let h = params.height;
        let mut pixels = vec![0; w * h * 3];

        // calculate pixel size
        let (pw, ph, rw) = {
            let pts = obj.faces.len();
            if pts > w * h {
                (1, 1, w)
            } else {
                // rw * rh = fs
                // rw / rh = w / h
                // rw = rh * w / h
                // rh ^ 2 * w / h = fs
                // rh = sqrt(fs * h / w)
                let rh = (pts as f32 * (h as f32 / w as f32)).sqrt() as usize;
                let rw = pts / rh;
                let pw = w / rw;
                let ph = h / rh;
                (pw, ph, rw)
            }
        };

        let mut pixels_comb = vec![(Vec3::new(), 0); w * h];
        match params.kind {
            ProjectionType::Face => {
                // TODO: maybe try sorting faces
                // currently we just visualize
                // patterns of how faces are sorted
                // inside the obj file.
                for (i, f) in obj.faces.iter().enumerate() {
                    let i = if i >= w * h { i % (w * h) } else { i };
                    let y = i / rw;
                    let x = i - rw * y;
                    let y = y * ph;
                    let x = x * pw;
                    for j in y..y + ph {
                        for k in x..x + pw {
                            let idx = j * w + k;
                            let mut v = Vec3::new();
                            for i in f.v.iter() {
                                let nv =
                                    (obj.vertices[*i].normalized() + Vec3::from_val(1.0)) / 2.0;
                                v += nv;
                            }
                            pixels_comb[idx] = (pixels_comb[idx].0 + v, pixels_comb[idx].1 + 3);
                        }
                    }
                }
            }
            ProjectionType::Vertex | ProjectionType::VertexNormal => {
                let pts = if params.kind == ProjectionType::Vertex {
                    &obj.vertices
                } else {
                    &obj.normals
                };
                for v in pts {
                    let v = v.normalized();
                    let v = (v + Vec3::from_val(1.0)) / 2.0;
                    let x = v.x;
                    let y = v.y;
                    let color = v.z;

                    let x = ((x * w as f32) as usize).min(w - 1);
                    let y = ((y * h as f32) as usize).min(h - 1);
                    let v = Vec3::from_val(color);
                    pixels_comb[y * h + x] =
                        (pixels_comb[y * h + x].0 + v, pixels_comb[y * h + x].1 + 1);
                }
            }
            ProjectionType::FaceNormal => {
                for (i, f) in obj.faces.iter().enumerate() {
                    let i = if i >= w * h { i % (w * h) } else { i };
                    let y = i / rw;
                    let x = i - rw * y;
                    let y = y * ph;
                    let x = x * pw;
                    for j in y..y + ph {
                        for k in x..x + pw {
                            let v0 = obj.vertices[f.v[0]];
                            let v1 = obj.vertices[f.v[1]];
                            let v2 = obj.vertices[f.v[2]];

                            let p1 = v0 - v1;
                            let p2 = v0 - v2;
                            let face_normal = p1.cross(&p2);
                            let vnormal = obj.normals[f.n[0]];
                            let face_normal = if face_normal.dot(&vnormal) < 0_f32 {
                                -face_normal
                            } else {
                                face_normal
                            };

                            let v = face_normal;

                            let idx = j * w + k;
                            pixels_comb[idx] = (pixels_comb[idx].0 + v, pixels_comb[idx].1 + 1);
                        }
                    }
                }
            }
        }

        for i in 0..w * h {
            let v = pixels_comb[i].0 / pixels_comb[i].1 as f32;
            pixels[i * 3] = (v.x * 255_f32) as u8;
            pixels[i * 3 + 1] = (v.y * 255_f32) as u8;
            pixels[i * 3 + 2] = (v.z * 255_f32) as u8;
        }

        Ok(pixels)
    }
}
