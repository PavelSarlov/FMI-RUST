use glam::f32::*;
use ggez::{
    mint::Point2,
    graphics::Color,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3Wrap(pub Vec3); 

impl Into<Point2<f32>> for Vec3Wrap {
    fn into(self) -> Point2<f32> {
        Point2::<f32> {
            x: self.0.x,
            y: self.0.y,
        }
    }
}

impl Into<Vec3> for Vec3Wrap {
    fn into(self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for Vec3Wrap {
    fn from(v: Vec3) -> Self {
        Vec3Wrap(v)
    }
}

impl From<Vec<f32>> for Vec3Wrap {
    fn from(v: Vec<f32>) -> Self {
        Vec3Wrap(Vec3::new(v[0], v[1], v[2]))
    }
}


#[derive(Clone, Debug)]
pub struct Triangle {
    pub verts: [Vec3Wrap; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(verts: &[Vec3Wrap; 3]) -> Self {
        Triangle {
            verts: verts.clone(),
            color: Color::new(1.0,1.0,1.0,1.0),
        }
    }
}

// the glam perspective method was kinda weird
pub fn perspective_lh(fov_rad: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::X * (aspect_ratio * fov_rad),
        Vec4::Y * fov_rad,
        Vec4::new(0.0, 0.0, (far / (far - near)), 1.0),
        Vec4::new(0.0, 0.0, (-far * near) / (far - near), 0.0),
    )
}

#[derive(Debug, Clone)]
pub struct TriMesh {
    pub tris: Vec<Triangle>, 
}

impl TriMesh {
    pub fn new(tris: Vec<Triangle>) -> Self {
        TriMesh {
            tris,
        }
    }

    pub fn load_from_obj_file(file_name: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(file_name))?;

        let mut vertices = Vec::<Vec3Wrap>::new();
        let mut triangles = Vec::<Triangle>::new();

        for line in BufReader::new(file).lines() {
            let l = line.unwrap();

            match l.chars().nth(0) {
                Some('v') => {
                    vertices.push(
                        l.trim_matches(|c| c == 'v' || c == ' ')
                            .split(' ')
                            .map(|c| c.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>()
                            .into(),
                    );
                },
                Some('f') => {
                    let indices = l.trim_matches(|c| c == 'f' || c == ' ')
                        .split(' ')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    triangles.push(
                        Triangle::new(&[vertices[indices[0]-1], vertices[indices[1]-1], vertices[indices[2]-1]])
                    );
                },
                _ => continue,
            }
        }
        Ok(TriMesh { 
            tris: triangles
        })
    }
}
