use encase::ShaderType;

use super::rotor::Rotor;

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct Point {
    pub e0123: f32,
    pub e0124: f32,
    pub e0134: f32,
    pub e0234: f32,
    pub e1234: f32,
}

impl Point {
    pub const IDENTITY: Point = Point {
        e0123: 0.0,
        e0124: 0.0,
        e0134: 0.0,
        e0234: 0.0,
        e1234: 1.0,
    };

    pub const fn from_cartesian(x: f32, y: f32, z: f32, w: f32) -> Self {
        Point {
            e0123: x,
            e0124: y,
            e0134: z,
            e0234: w,
            e1234: 1.0,
        }
    }

    pub fn transform(self, _rotor: Rotor) -> Self {
        todo!()
    }
}

/*

(
    a +
    b*e0*e1 +
    c*e0*e2 +
    d*e0*e3 +
    f*e0*e4 +
    g*e1*e2 +
    h*e1*e3 +
    i*e1*e4 +
    j*e2*e3 +
    k*e2*e4 +
    l*e3*e4 +
    m*e0*e1*e2*e3 +
    n*e0*e1*e2*e4 +
    o*e0*e1*e3*e4 +
    p*e0*e2*e3*e4 +
    q*e1*e2*e3*e4
)
*
(
    r*e0123
    s*e0124
    t*e0134
    u*e0234
    v*e1234
)
*
(
    a +
    b*e1*e0 +
    c*e2*e0 +
    d*e3*e0 +
    f*e4*e0 +
    g*e2*e1 +
    h*e3*e1 +
    i*e4*e1 +
    j*e3*e2 +
    k*e4*e2 +
    l*e4*e3 +
    m*e3*e2*e1*e0 +
    n*e4*e2*e1*e0 +
    o*e4*e3*e1*e0 +
    p*e4*e3*e2*e0 +
    q*e4*e3*e2*e1
)

// https://enki.ws/ganja.js/examples/coffeeshop.html#RdnNerHrK

*/
