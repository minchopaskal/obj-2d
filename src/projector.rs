use crate::{
    err::ProjectorResult,
    obj::{Obj, Vert},
};

pub struct Projector {}

impl Projector {
    pub fn new() -> Projector {
        Projector {}
    }

    pub fn project(&self, obj: Obj, w: usize, h: usize) -> ProjectorResult<Vec<u8>> {
        let mut pixels = vec![0; w * h * 3];

        // calculate pixel size
        let (pw, ph, rw) = {
            let fs = obj.faces.len();
            if fs > w * h {
                (1, 1, w)
            } else {
                // rw * rh = fs
                // rw / rh = w / h
                // rw = rh * w / h
                // rh ^ 2 * w / h = fs
                // rh = sqrt(fs * h / w)
                let rh = (fs as f32 * (h as f32 / w as f32)).sqrt() as usize;
                let rw = fs / rh;
                let pw = w / rw;
                let ph = h / rh;
                (pw, ph, rw)
            }
        };

        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for v in obj.vertices.iter() {
            min = min.min(v.x);
            min = min.min(v.y);
            min = min.min(v.z);

            max = max.max(v.x);
            max = max.max(v.y);
            max = max.max(v.z);
        }

        let rng = max - min;

        let mut pixels_comb = vec![(Vert::new(), 0); w * h];
        for (i, f) in obj.faces.iter().enumerate() {
            let i = if i >= w * h { i % (w * h) } else { i };
            let y = i / rw;
            let x = i - rw * y;
            let y = y * ph;
            let x = x * pw;
            for j in y..y + ph {
                for k in x..x + pw {
                    let idx = j * w + k;
                    let mut v = Vert::new();
                    for i in f.iter() {
                        v += (obj.vertices[*i as usize] + min.abs()) / rng;
                        v /= 3_f32;
                    }
                    pixels_comb[idx] = (pixels_comb[idx].0 + v, pixels_comb[idx].1 + 1);
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
