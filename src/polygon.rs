use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm::Vec3;

pub trait Polygon {
    fn polygon(&mut self, points: &[Vec3]);
    fn fill_polygon(&mut self, points: &[Vec3]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[Vec3]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let v1 = points[i];
            let v2 = points[(i + 1) % points.len()];

            self.line(v1, v2);
        }
    }

    fn fill_polygon(&mut self, vertices: &[Vec3]) {
        if vertices.len() < 3 {
            return;
        }

        let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).round() as isize;
        let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).round() as isize;

        for y in min_y..=max_y {
            let mut intersections = vec![];

            for i in 0..vertices.len() {
                let start = vertices[i];
                let end = if i == vertices.len() - 1 {
                    vertices[0]
                } else {
                    vertices[i + 1]
                };

                if (start.y.round() as isize <= y && end.y.round() as isize > y) || (end.y.round() as isize <= y && start.y.round() as isize > y) {
                    let x = start.x + (y as f32 - start.y) * (end.x - start.x) / (end.y - start.y);
                    intersections.push(x.round() as isize);
                }
            }

            intersections.sort();

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    for x in intersections[i]..=intersections[i + 1] {
                        self.point(x as usize, y as usize);
                    }
                }
            }
        }
    }
}
