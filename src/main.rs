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

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        
        Vec3::new(321.0, 335.0, 0.0), Vec3::new(288.0, 286.0, 0.0), Vec3::new(339.0, 251.0, 0.0), Vec3::new(374.0, 302.0, 0.0)
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0xFF0000);
    framebuffer.fill_polygon(&polygon_points);

    let _ = framebuffer.render_buffer("output.bmp");
}