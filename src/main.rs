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

    //Polígono 1
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


    //Polígono 2
    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0xFF0000);
    framebuffer.fill_polygon(&polygon_points);


    //Polígono 3
    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0)
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0x0000FF);
    framebuffer.fill_polygon(&polygon_points);

    //Polígono 4
    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0x00FF00);
    framebuffer.fill_polygon(&polygon_points);

    //Polígono 5 (Agujero)
    framebuffer.set_current_color(0xFFFFFF);

    let polygon_points = vec![
        Vec3::new(682.0, 175.0, 0.0),  Vec3::new(708.0, 120.0, 0.0),  Vec3::new(735.0, 148.0, 0.0),  Vec3::new(739.0, 170.0, 0.0)
    ];

    framebuffer.polygon(&polygon_points);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.fill_polygon(&polygon_points);

    let _ = framebuffer.render_buffer("output.bmp");
}