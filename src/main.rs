extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        Vec3::new(165.0, 380.0, 0.0), Vec3::new(185.0, 360.0, 0.0), Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0), Vec3::new(233.0, 330.0, 0.0), Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0), Vec3::new(220.0, 385.0, 0.0), Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0x00FFFF);
    framebuffer.fill_polygon(&polygon_points);

    let _ = framebuffer.render_buffer("output.bmp");
}
