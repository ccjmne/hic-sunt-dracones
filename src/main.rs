use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;
use std::{char, fmt, io};
use std::fs::File;
use std::io::{stdout, Read, Write};

//fn at(lat: i16, long: i16, map: String, w: i16, h: i16) {
//    let y = ( lat % 181 ) - 90;
//    //let y = [-90, 90] => [lines-1, 0]
//    println!("{}", y);
//    println!("{}", map.chars().nth(lat.try_into().unwrap()).unwrap());
//}

//fn project(x: u16, y: u16) -> char {
//    'x'
//}

struct Frustum {
    _fov:  f32,
    _ar:   f32,
    _near: f32,
    _far:  f32,
}

struct Camera {
    _frustum:   Frustum,
    _azimuth:   f32,
    _elevation: f32,
    _distance:  f32,
}

struct Coords {
    lat:  f32,
    long: f32,
}
impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.long)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

use std::ops::{Add, Sub, Mul};

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

//// The scene consists of a single sphere located around (0, 0), of radius 1.
//fn perspective(_camera: &Camera) -> Option<Coords> {
//    char::from_u32(0x2800 + random::<u8>() as u32).unwrap();
//    return Some(Coords { lat: 0., long: 0. })
//}

// 
// 
// 
// x^2 + y^2 + z^2 = 1
// x^2 = 1 - y^2 - z^2
// x = ±sqrt(1 - y^2 - z^2)
//
//
// x0 + t*a = ±sqrt(1 - (y0 + t*b)^2 - (z0 + t*c)^2)
//
//
// USE THIS ONE!!!!
// (x0 + t*a)^2 + (y0 + t*b)^2 + (z0 + t*c)^2 = 1
//
// let's fix x0 and y0 to 0
// (t*a)^2 + (t*b)^2 + (z0 + t*c)^2 = 1
// t^2 * a^2 + t^2 * b^2 + (z0 + t * c)^2 = 1
// t^2 (a^2 + b^2) + z0^2 + 2 * z0 * t * c + t^2*c^2 = 1
// t^2 (a^2 + b^2 + c^2) + (2 * z0 * c) * t + (z0^2 - 1) = 0
//
// A = a^2 + b^2 + c^2
// B = 2 * z0 * c
// C = z0^2 - 1
//
// t = ( -B ± sqrt(B^2 - 4AC) ) / 2A
// 
// if (B^2 - 4AC) < 0, no solutions
// if (B^2 - 4AC) = 0, one solution
// if (B^2 - 4AC) > 0, two solutions
// 
// t = ( -(2 * z0 * c) ± sqrt((2 * z0 * c)^2 - 4*(a^2 + b^2 + c^2)*(z0^2 - 1)) ) / ( 2 * ( a^2 + b^2 + c^2 ) )
// t = ( -(2 * z0 * c) ± sqrt(4[(z0 * c)^2 - (a^2 + b^2 + c^2)*(z0^2 - 1)]) ) / ( 2 * ( a^2 + b^2 + c^2 ) )
// t = ( -2 * (z0 * c) ± 2 * sqrt((z0 * c)^2 - (a^2 + b^2 + c^2)*(z0^2 - 1)) ) / ( 2 * ( a^2 + b^2 + c^2 ) )
//
// t = ( -z0 * c ± sqrt((z0 * c)^2 - (a^2 + b^2 + c^2)*(z0^2 - 1)) ) / ( a^2 + b^2 + c^2 )
//
// 
// Basically, if there exists a `t` that works, you're on the line.
// (x-x0) / a = (y-y0) / b = (z-z0) / c
//
//
//
//
// (x-x0) / a = (y-y0) / b = (z-z0) / c
//
fn isect(Vec3 { z: z0, .. }: Vec3, Vec3 { x: a, y: b, z: c }: Vec3) -> Option<Vec3> {
    // Equation of a sphere at (0, 0, 0) of radius 1
    // (x-x0)^2 + (y-y0)^2 + (z-z0)^2 = r^2
    // Let's fix x0, y0, z0 = 0, and r = 1
    //
    // x^2 + y^2 + z^2 = 1
    //
    // Equation for a line going through (x, y, z) and towards (a, b, c)
    // (x, y, z) = (x0, y0, z0) + t(a, b, c)
    // t is any real value
    //
    // x = x0 + t * a
    // y = y0 + t * b
    // z = z0 + t * b
    //
    // Let's fix x0, y0 = 0
    // (t*a)^2 + (t*b)^2 + (z0 + t*c)^2 = 1
    // t^2 (a^2 + b^2 + c^2) + (2 * z0 * c) * t + (z0^2 - 1) = 0
    // ^^^ there's our quadratic equation to solve
    let A = a.powi(2)  + b.powi(2)  + c.powi(2) ;
    let B = 2.  * z0 * c;
    let C = z0.powi(2)  - 1. ;
    // Just get the lowest value of t, the nearest to the camera
    let t = if B.powi(2) - 4. * A * C < 0. { None }
            else                           { Some( ( -B - (B.powi(2)  - 4. * A * C).sqrt() ) / ( 2. * A ) )};

    //if t.is_some() {
    //    println!("intersect: {}", Vec3 { x: a, y: b, z: c});
    //} else {
    //    println!("doesn't intersect: {}", Vec3 { x: a, y: b, z: c})
    //}

    t.map(|tt| Vec3 {x: 0., y: 0., z: z0} + Vec3{ x: a, y: b, z: c} * tt)
}

fn toGeometric(Vec3 { x, y, z }: Vec3) -> Coords {
    //println!("toGeometric: {}", z);
    fn angle(x: f32, y: f32, dot_product: f32) -> f32 {
        //let magnitude_a = 1.0; // Magnitude of (0, -1) is 1
        let magnitude_b = (x * x + y * y).sqrt();
        ( dot_product / magnitude_b ).acos()
    }

    // dot_product is -y for (-1, 0) dot (y, z), -z for (0, -1) dot (x, z)
    Coords {
        // lat is the angle between (-1, 0) and (y, z) on the yz plane
        lat: angle(y, z, -y),
        // long is the angle between (0, -1) and (x, z) on the xz plane
        long: angle(x, z, -x),
    }
}

fn texture3(coords: Coords, rot: f32, map: &String) -> char {
    //println!("texture2: {}", coords);

    let long = ( ( coords.long + rot ) + ( PI * 2. ) ) % ( PI * 2. );
    //let long = coords.long;
    //let lat = ( coords.lat + rot ) % ( PI );
    let lat = coords.lat;

    let lines: Vec<&str> = map.split('\n').collect();

    let w = map[..map.find('\n').unwrap()].chars().count() as f32;
    let h = map.chars().filter(|&c| c == '\n').count() as f32;

    let x = long * w / (2. * PI); // [0, 2 * PI[ -> [0, w[
    let y = ( lat + 0.) * h / PI; // [- PI / 2, PI / 2[ -> [0, h[
    
    return lines[h as usize - y as usize].chars().nth(x as usize).unwrap_or(' ');

    //if long % (PI / 6.) < 0.05 * PI / 12. {
    //    char::from_u32(0x2800 + 0b1011_1000).unwrap()
    //} else
    if long % (PI / 6.) < PI / 12. {
        //return '\u{28ff}';
        return match y / h {
            r if r < 0.2 => char::from_u32(0x2800 + 0b0000_0000).unwrap(),
            r if r < 0.4 => char::from_u32(0x2800 + 0b0000_1001).unwrap(),
            r if r < 0.6 => char::from_u32(0x2800 + 0b0001_1011).unwrap(),
            r if r < 0.8 => char::from_u32(0x2800 + 0b0011_1111).unwrap(),
            _            => char::from_u32(0x2800 + 0b1111_1111).unwrap(),
        }
    //} else if long % (PI / 6.) < PI / 12. {
    //    char::from_u32(0x2800 + 0b0100_0111).unwrap()
    } else {
        '\u{2800}'
    }
}

fn texture2(coords: Coords, rot: f32) -> char {
    //println!("texture2: {}", coords);
    let long = coords.long + rot;
    if long % (PI / 6.) < 0.05 * PI / 12. {
        char::from_u32(0x2800 + 0b1011_1000).unwrap()
    } else if long % (PI / 6.) < 0.95 * PI / 12. {
        '\u{28ff}'
    } else if long % (PI / 6.) < PI / 12. {
        char::from_u32(0x2800 + 0b0100_0111).unwrap()
    } else {
        '\u{2800}'
    }
}

fn texture(coords: Option<Coords>) -> char {
    match coords {
        None => ' ',
        Some(Coords { long, .. }) => 
        if long % (PI / 6.) < 0.05 * PI / 12. {
            char::from_u32(0x2800 + 0b1011_1000).unwrap()
        } else if long % (PI / 6.) < 0.95 * PI / 12. {
            '\u{28ff}'
        } else if long % (PI / 6.) < PI / 12. {
            char::from_u32(0x2800 + 0b0100_0111).unwrap()
        } else {
            '\u{2800}'
        }
    }
}

fn main() {
    let Ok(mut file) = File::open("./data/s") else { return };

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    //println!("{}", contents);
    let mut w: u8 = contents[..contents.find('\n').unwrap()].chars().count() as u8;

    //let camera = Camera {
    //    _frustum: Frustum {
    //        _fov:  PI / 2.,
    //        //_fov:  90.0_f32.to_radians(),
    //        _ar:   16. / 9.,
    //        _near: 0.1,
    //        _far:  100.,
    //    },
    //    _azimuth:   0.,
    //    _elevation: PI / 4.,
    //    //_elevation: 45.0_f32.to_radians(),
    //    _distance:  2.,
    //};

    w *= 1;
    fn draw(w: u8, rot: f32, map: &String) -> Vec<u8> {
        let mut buffer = Vec::new();
        let wf = w as f32;
        for y in 0..w/4 {
            let yf = y as f32;
            for x in 0..w {
                let xf = x as f32;
                let pixel = isect(Vec3 {x: 0., y: 0., z: -1.5}, Vec3 {
                    x: xf * 4./ wf - 2.,// [0, w[ -> [-2, +2[
                    y: yf * -2. / (wf/4.) + 1.,// [0, w/2[ -> [1, -1[
                    z: 1.0,
                }).map(toGeometric).map_or(' ', |asdf| texture3(asdf, rot, map));
                write!(buffer, "{}", pixel);
            }
            write!(buffer, "\n");
        }
        buffer
    }

    let mut rot = 0.;
    loop {
        let buf = draw(w, rot, &contents);
        let _ = stdout().write(&buf);
        let _ = stdout().flush();
        rot += PI / 30.;
        sleep(Duration::from_millis(1000 / 60));
        print!("\r\x1B[{}A", buf.iter().filter(|&&c| c == b'\n').count());
    }

    //let wf = w as f32;
    //let xf = wf/2.;
    //let yf = wf/4./2.;
    //let pixel = isect(Vec3 {x: 0., y: 0., z: -3.}, Vec3 {
    //    x: xf * 4./ wf - 2.,// [0, w[ -> [-2, +2[
    //    y: yf * -2. / (wf/4.) + 1.,// [0, w/2[ -> [1, -1[
    //    z: 1.,
    //});
    //if (pixel.is_some()) {
    //    println!("{}", pixel.unwrap())
    //} else {
    //    println!("nope")
    //}

    // ]123456789012345123456789012345123456789012345612345678901234[
    // ]###############[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]################[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]
    // ]⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]
    // ]⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]
    // ]⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇[⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀]

    //println!("{}", perspective(&camera).unwrap());
    //for x in 0..w {
    //    let long = x as f32 * 2. * PI / w as f32;
    //    print!("{}", texture(Some(Coords { lat: 0., long: long })))
    //}

    
    //loop {
    //    for _ in 0..w {
    //        print!("{}", char::from_u32(0x2800 + random::<u8>() as u32).unwrap());
    //    }
    //    std::thread::sleep(std::time::Duration::from_millis(1000 / w as u32));
    //    print!("\n\r\x1B[1A");
    //}

    //at(0, 0, contents, w, w / 2);
}
