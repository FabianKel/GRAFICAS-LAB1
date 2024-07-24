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

    //Polígono 1
    //Se definen los vectores
    let polygon_points = vec![
        Vec3::new(165.0, 380.0, 0.0), Vec3::new(185.0, 360.0, 0.0), Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0), Vec3::new(233.0, 330.0, 0.0), Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0), Vec3::new(220.0, 385.0, 0.0), Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    //Se rellena primero y se define el color
    framebuffer.set_current_color(0x00FFFF); //(G,B,R) -> Amarillo
    framebuffer.fill_polygon(&polygon_points);

    //Se traza el borde después
    framebuffer.set_current_color(0xFFFFFF);//Borde Blanco
    framebuffer.polygon(&polygon_points);



    //Polígono 2

    //Se definen los vectores
    let polygon_points = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];

    //Se rellena primero y se define el color
    framebuffer.set_current_color(0xFF0000); //(G,B,R) -> Verde
    framebuffer.fill_polygon(&polygon_points);

    //Se traza el borde después
    framebuffer.set_current_color(0xFFFFFF);//Borde Blanco
    framebuffer.polygon(&polygon_points);

    let _ = framebuffer.render_buffer("output.bmp");
}