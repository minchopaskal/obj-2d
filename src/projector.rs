use crate::{
    err::ProjectorResult,
    obj::{Obj, Vec3},
};

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
    const EPSILON: f32 = 1e-6;

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

        let mut vmin = Vec3::from_val(f32::MAX);
        let mut vmax = Vec3::from_val(f32::MIN);
        for v in obj.vertices.iter() {
            vmin.x = vmin.x.min(v.x);
            vmin.y = vmin.y.min(v.y);
            vmin.z = vmin.z.min(v.z);
            vmax.x = vmax.x.max(v.x);
            vmax.y = vmax.y.max(v.y);
            vmax.z = vmax.z.max(v.z);
        }
        let vrng = vmax - vmin;

        let mut nmin = Vec3::from_val(f32::MAX);
        let mut nmax = Vec3::from_val(f32::MIN);
        for v in obj.normals.iter() {
            nmin.x = nmin.x.min(v.x);
            nmin.y = nmin.y.min(v.y);
            nmin.z = nmin.z.min(v.z);
            nmax.x = nmax.x.max(v.x);
            nmax.y = nmax.y.max(v.y);
            nmax.z = nmax.z.max(v.z);
        }
        let nrng = nmax - nmin;

        println!("{nmin:?} {nmax:?} {nrng:?}");

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
                            for (n, i) in f.v.iter().enumerate() {
                                v += (obj.vertices[*i] + vmin.get(n).abs()) / vrng.get(n);
                                v /= 3_f32;
                            }
                            pixels_comb[idx] = (pixels_comb[idx].0 + v, pixels_comb[idx].1 + 1);
                        }
                    }
                }
            }
            ProjectionType::Vertex => {
                for v in &obj.vertices {
                    let x = (v.x - vmin.x) / vrng.x;
                    let y = (v.y - vmin.y) / vrng.y;

                    let color = if v.z.abs() < Projector::EPSILON {
                        0_f32
                    } else {
                        (v.z.abs() - vmin.z) / vrng.z
                    };

                    let x = ((x * w as f32) as usize).min(w - 1);
                    let y = ((y * h as f32) as usize).min(h - 1);
                    let v = Vec3::from_val(color);
                    pixels_comb[y * h + x] =
                        (pixels_comb[y * h + x].0 + v, pixels_comb[y * h + x].1 + 1);
                }
            }
            ProjectionType::VertexNormal => {
                for v in obj.normals.iter() {
                    let x = (v.x - nmin.x) / nrng.x;
                    let y = (v.y - nmin.y) / nrng.y;

                    let z = if v.z.abs() < Projector::EPSILON {
                        0_f32
                    } else {
                        (v.z.abs() - nmin.z) / nrng.z
                    };

                    let x = ((x * w as f32) as usize).min(w - 1);
                    let y = ((y * h as f32) as usize).min(h - 1);
                    assert!(x < w, "{x} !< {w}");
                    assert!(y < h, "{y} !< {h}");
                    let v = Vec3::from_val(z);
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
