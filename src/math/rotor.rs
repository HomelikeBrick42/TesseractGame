use std::ops::{Mul, Not};

use encase::ShaderType;

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct Rotor {
    pub s: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e04: f32,
    pub e12: f32,
    pub e13: f32,
    pub e14: f32,
    pub e23: f32,
    pub e24: f32,
    pub e34: f32,
    pub e0123: f32,
    pub e0124: f32,
    pub e0134: f32,
    pub e0234: f32,
    pub e1234: f32,
}

impl Rotor {
    pub const IDENTITY: Rotor = Rotor {
        s: 1.0,
        e01: 0.0,
        e02: 0.0,
        e03: 0.0,
        e04: 0.0,
        e12: 0.0,
        e13: 0.0,
        e14: 0.0,
        e23: 0.0,
        e24: 0.0,
        e34: 0.0,
        e0123: 0.0,
        e0124: 0.0,
        e0134: 0.0,
        e0234: 0.0,
        e1234: 0.0,
    };

    pub fn magnitude_squared(self) -> f32 {
        (!self * self).s
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> Rotor {
        let inverse_magnitude = self.magnitude().recip();
        let Rotor {
            s,
            e01,
            e02,
            e03,
            e04,
            e12,
            e13,
            e14,
            e23,
            e24,
            e34,
            e0123,
            e0124,
            e0134,
            e0234,
            e1234,
        } = self;
        Rotor {
            s: s * inverse_magnitude,
            e01: e01 * inverse_magnitude,
            e02: e02 * inverse_magnitude,
            e03: e03 * inverse_magnitude,
            e04: e04 * inverse_magnitude,
            e12: e12 * inverse_magnitude,
            e13: e13 * inverse_magnitude,
            e14: e14 * inverse_magnitude,
            e23: e23 * inverse_magnitude,
            e24: e24 * inverse_magnitude,
            e34: e34 * inverse_magnitude,
            e0123: e0123 * inverse_magnitude,
            e0124: e0124 * inverse_magnitude,
            e0134: e0134 * inverse_magnitude,
            e0234: e0234 * inverse_magnitude,
            e1234: e1234 * inverse_magnitude,
        }
    }
}

impl Not for Rotor {
    type Output = Rotor;

    fn not(self) -> Self::Output {
        let Rotor {
            s,
            e01,
            e02,
            e03,
            e04,
            e12,
            e13,
            e14,
            e23,
            e24,
            e34,
            e0123,
            e0124,
            e0134,
            e0234,
            e1234,
        } = self;
        Rotor {
            s,
            e01: -e01,
            e02: -e02,
            e03: -e03,
            e04: -e04,
            e12: -e12,
            e13: -e13,
            e14: -e14,
            e23: -e23,
            e24: -e24,
            e34: -e34,
            e0123,
            e0124,
            e0134,
            e0234,
            e1234,
        }
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, rhs: Rotor) -> Self::Output {
        let Rotor {
            s: a1,
            e01: b1,
            e02: c1,
            e03: d1,
            e04: f1,
            e12: g1,
            e13: h1,
            e14: i1,
            e23: j1,
            e24: k1,
            e34: l1,
            e0123: m1,
            e0124: n1,
            e0134: o1,
            e0234: p1,
            e1234: q1,
        } = self;
        let Rotor {
            s: a2,
            e01: b2,
            e02: c2,
            e03: d2,
            e04: f2,
            e12: g2,
            e13: h2,
            e14: i2,
            e23: j2,
            e24: k2,
            e34: l2,
            e0123: m2,
            e0124: n2,
            e0134: o2,
            e0234: p2,
            e1234: q2,
        } = rhs;
        Rotor {
            s: -g1 * g2 + -h1 * h2 + -i1 * i2 + -j1 * j2 + -k1 * k2 + -l1 * l2 + a1 * a2 + q1 * q2,
            e01: -c1 * g2
                + -d1 * h2
                + -f1 * i2
                + -j1 * m2
                + -j2 * m1
                + -k1 * n2
                + -k2 * n1
                + -l1 * o2
                + -l2 * o1
                + -p2 * q1
                + a1 * b2
                + a2 * b1
                + c2 * g1
                + d2 * h1
                + f2 * i1
                + p1 * q2,
            e02: -b2 * g1
                + -d1 * j2
                + -f1 * k2
                + -l1 * p2
                + -l2 * p1
                + -o1 * q2
                + a1 * c2
                + a2 * c1
                + b1 * g2
                + d2 * j1
                + f2 * k1
                + h1 * m2
                + h2 * m1
                + i1 * n2
                + i2 * n1
                + o2 * q1,
            e03: -b2 * h1
                + -c2 * j1
                + -f1 * l2
                + -g1 * m2
                + -g2 * m1
                + -n2 * q1
                + a1 * d2
                + a2 * d1
                + b1 * h2
                + c1 * j2
                + f2 * l1
                + i1 * o2
                + i2 * o1
                + k1 * p2
                + k2 * p1
                + n1 * q2,
            e04: -b2 * i1
                + -c2 * k1
                + -d2 * l1
                + -g1 * n2
                + -g2 * n1
                + -h1 * o2
                + -h2 * o1
                + -j1 * p2
                + -j2 * p1
                + -m1 * q2
                + a1 * f2
                + a2 * f1
                + b1 * i2
                + c1 * k2
                + d1 * l2
                + m2 * q1,
            e12: -h1 * j2 + -i1 * k2 + -l1 * q2 + -l2 * q1 + a1 * g2 + a2 * g1 + h2 * j1 + i2 * k1,
            e13: -g2 * j1 + -i1 * l2 + a1 * h2 + a2 * h1 + g1 * j2 + i2 * l1 + k1 * q2 + k2 * q1,
            e14: -g2 * k1 + -h2 * l1 + -j1 * q2 + -j2 * q1 + a1 * i2 + a2 * i1 + g1 * k2 + h1 * l2,
            e23: -g1 * h2 + -i1 * q2 + -i2 * q1 + -k1 * l2 + a1 * j2 + a2 * j1 + g2 * h1 + k2 * l1,
            e24: -g1 * i2 + -j2 * l1 + a1 * k2 + a2 * k1 + g2 * i1 + h1 * q2 + h2 * q1 + j1 * l2,
            e34: -g1 * q2 + -g2 * q1 + -h1 * i2 + -j1 * k2 + a1 * l2 + a2 * l1 + h2 * i1 + j2 * k1,
            e0123: -c1 * h2
                + -c2 * h1
                + -f1 * q2
                + -i2 * p1
                + -k1 * o2
                + -l2 * n1
                + a1 * m2
                + a2 * m1
                + b1 * j2
                + b2 * j1
                + d1 * g2
                + d2 * g1
                + f2 * q1
                + i1 * p2
                + k2 * o1
                + l1 * n2,
            e0124: -c1 * i2
                + -c2 * i1
                + -d2 * q1
                + -h1 * p2
                + -j2 * o1
                + -l1 * m2
                + a1 * n2
                + a2 * n1
                + b1 * k2
                + b2 * k1
                + d1 * q2
                + f1 * g2
                + f2 * g1
                + h2 * p1
                + j1 * o2
                + l2 * m1,
            e0134: -c1 * q2
                + -d1 * i2
                + -d2 * i1
                + -g2 * p1
                + -j1 * n2
                + -k2 * m1
                + a1 * o2
                + a2 * o1
                + b1 * l2
                + b2 * l1
                + c2 * q1
                + f1 * h2
                + f2 * h1
                + g1 * p2
                + j2 * n1
                + k1 * m2,
            e0234: -b2 * q1
                + -d1 * k2
                + -d2 * k1
                + -g1 * o2
                + -h2 * n1
                + -i1 * m2
                + a1 * p2
                + a2 * p1
                + b1 * q2
                + c1 * l2
                + c2 * l1
                + f1 * j2
                + f2 * j1
                + g2 * o1
                + h1 * n2
                + i2 * m1,
            e1234: -h1 * k2 + -h2 * k1 + a1 * q2 + a2 * q1 + g1 * l2 + g2 * l1 + i1 * j2 + i2 * j1,
        }
    }
}

/*
(
    a1 +
    b1*e0*e1 +
    c1*e0*e2 +
    d1*e0*e3 +
    f1*e0*e4 +
    g1*e1*e2 +
    h1*e1*e3 +
    i1*e1*e4 +
    j1*e2*e3 +
    k1*e2*e4 +
    l1*e3*e4 +
    m1*e0*e1*e2*e3 +
    n1*e0*e1*e2*e4 +
    o1*e0*e1*e3*e4 +
    p1*e0*e2*e3*e4 +
    q1*e1*e2*e3*e4
)
*
(
    a2 +
    b2*e0*e1 +
    c2*e0*e2 +
    d2*e0*e3 +
    f2*e0*e4 +
    g2*e1*e2 +
    h2*e1*e3 +
    i2*e1*e4 +
    j2*e2*e3 +
    k2*e2*e4 +
    l2*e3*e4 +
    m2*e0*e1*e2*e3 +
    n2*e0*e1*e2*e4 +
    o2*e0*e1*e3*e4 +
    p2*e0*e2*e3*e4 +
    q2*e1*e2*e3*e4
)

  (-1*g1*g2 + -1*h1*h2 + -1*i1*i2 + -1*j1*j2 + -1*k1*k2 + -1*l1*l2 + a1*a2 + q1*q2)
+ (-1*c1*g2 + -1*d1*h2 + -1*f1*i2 + -1*j1*m2 + -1*j2*m1 + -1*k1*n2 + -1*k2*n1 + -1*l1*o2 + -1*l2*o1 + -1*p2*q1 + a1*b2 + a2*b1 + c2*g1 + d2*h1 + f2*i1 + p1*q2)*e0*e1
+ (-1*b2*g1 + -1*d1*j2 + -1*f1*k2 + -1*l1*p2 + -1*l2*p1 + -1*o1*q2 + a1*c2 + a2*c1 + b1*g2 + d2*j1 + f2*k1 + h1*m2 + h2*m1 + i1*n2 + i2*n1 + o2*q1)*e0*e2
+ (-1*b2*h1 + -1*c2*j1 + -1*f1*l2 + -1*g1*m2 + -1*g2*m1 + -1*n2*q1 + a1*d2 + a2*d1 + b1*h2 + c1*j2 + f2*l1 + i1*o2 + i2*o1 + k1*p2 + k2*p1 + n1*q2)*e0*e3
+ (-1*b2*i1 + -1*c2*k1 + -1*d2*l1 + -1*g1*n2 + -1*g2*n1 + -1*h1*o2 + -1*h2*o1 + -1*j1*p2 + -1*j2*p1 + -1*m1*q2 + a1*f2 + a2*f1 + b1*i2 + c1*k2 + d1*l2 + m2*q1)*e0*e4
+ (-1*h1*j2 + -1*i1*k2 + -1*l1*q2 + -1*l2*q1 + a1*g2 + a2*g1 + h2*j1 + i2*k1)*e1*e2
+ (-1*g2*j1 + -1*i1*l2 + a1*h2 + a2*h1 + g1*j2 + i2*l1 + k1*q2 + k2*q1)*e1*e3
+ (-1*g2*k1 + -1*h2*l1 + -1*j1*q2 + -1*j2*q1 + a1*i2 + a2*i1 + g1*k2 + h1*l2)*e1*e4
+ (-1*g1*h2 + -1*i1*q2 + -1*i2*q1 + -1*k1*l2 + a1*j2 + a2*j1 + g2*h1 + k2*l1)*e2*e3
+ (-1*g1*i2 + -1*j2*l1 + a1*k2 + a2*k1 + g2*i1 + h1*q2 + h2*q1 + j1*l2)*e2*e4
+ (-1*g1*q2 + -1*g2*q1 + -1*h1*i2 + -1*j1*k2 + a1*l2 + a2*l1 + h2*i1 + j2*k1)*e3*e4
+ (-1*c1*h2 + -1*c2*h1 + -1*f1*q2 + -1*i2*p1 + -1*k1*o2 + -1*l2*n1 + a1*m2 + a2*m1 + b1*j2 + b2*j1 + d1*g2 + d2*g1 + f2*q1 + i1*p2 + k2*o1 + l1*n2)*e0*e1*e2*e3
+ (-1*c1*i2 + -1*c2*i1 + -1*d2*q1 + -1*h1*p2 + -1*j2*o1 + -1*l1*m2 + a1*n2 + a2*n1 + b1*k2 + b2*k1 + d1*q2 + f1*g2 + f2*g1 + h2*p1 + j1*o2 + l2*m1)*e0*e1*e2*e4
+ (-1*c1*q2 + -1*d1*i2 + -1*d2*i1 + -1*g2*p1 + -1*j1*n2 + -1*k2*m1 + a1*o2 + a2*o1 + b1*l2 + b2*l1 + c2*q1 + f1*h2 + f2*h1 + g1*p2 + j2*n1 + k1*m2)*e0*e1*e3*e4
+ (-1*b2*q1 + -1*d1*k2 + -1*d2*k1 + -1*g1*o2 + -1*h2*n1 + -1*i1*m2 + a1*p2 + a2*p1 + b1*q2 + c1*l2 + c2*l1 + f1*j2 + f2*j1 + g2*o1 + h1*n2 + i2*m1)*e0*e2*e3*e4
+ (-1*h1*k2 + -1*h2*k1 + a1*q2 + a2*q1 + g1*l2 + g2*l1 + i1*j2 + i2*j1)*e1*e2*e3*e4
*/
