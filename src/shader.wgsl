@group(0)
@binding(0)
var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Camera {
    transform: Rotor,
    v_fov: f32,
}

@group(1)
@binding(0)
var<uniform> camera: Camera;

struct Ray {
    origin: vec4<f32>,
    direction: vec4<f32>,
}

struct Hit {
    hit: bool,
    position: vec4<f32>,
    normal: vec4<f32>,
    color: vec3<f32>,
}

struct Block {
    color: vec3<f32>,
    exists: u32,
}

struct Chunk {
    data: array<Block, 256>,
}

@group(2)
@binding(0)
var<storage, read> chunk: Chunk;

fn trace_ray(ray: Ray) -> Hit {
    var hit: Hit;
    hit.hit = false;

    let step_sizes = 1.0 / abs(ray.direction);
    let step_dir = vec4<i32>(sign(ray.direction));
    var next_dist = (vec4<f32>(step_dir) * 0.5 + 0.5 - fract(ray.origin)) / ray.direction;

    var curr_pos = ray.origin;
    var voxel_pos = vec4<i32>(floor(curr_pos));
    for (var i = 0u; i < 100u; i += 1u) {
        let closest_dist = min(min(min(next_dist.x, next_dist.y), next_dist.z), next_dist.w);
        curr_pos += ray.direction * closest_dist;
        let step_axis = vec4<i32>(next_dist == vec4<f32>(closest_dist));
        voxel_pos += step_axis * step_dir;
        next_dist -= closest_dist;
        next_dist += step_sizes * vec4<f32>(step_axis);

        if all(voxel_pos >= vec4<i32>(0)) && all(voxel_pos < vec4<i32>(4)) {
            let index = u32(voxel_pos.x) + u32(voxel_pos.y) * 4 + u32(voxel_pos.z) * 4 * 4 + u32(voxel_pos.w) * 4 * 4 * 4;
            if chunk.data[index].exists != 0 {
                hit.hit = true;
                hit.position = curr_pos;
                hit.normal = vec4<f32>(-step_axis * step_dir);
                hit.color = chunk.data[index].color;
                return hit;
            }
        }
    }

    return hit;
}

@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let size = textureDimensions(output_texture);
    let coords = global_id.xy;

    if coords.x >= size.x || coords.y >= size.y {
        return;
    }

    let theta = tan(camera.v_fov / 2.0);
    let aspect = f32(size.x) / f32(size.y);
    let normalized_uv = vec2<f32>(f32(coords.x) / f32(size.x), 1.0 - (f32(coords.y) / f32(size.y))) * 2.0 - 1.0;

    var ray: Ray;
    ray.origin = transform(camera.transform, vec4<f32>(0.0, 0.0, 0.0, 0.0));
    ray.direction = normalize(transform_direction(camera.transform, vec4<f32>(1.0, normalized_uv.y * theta, normalized_uv.x * aspect * theta, 0.0)));

    var color = vec3<f32>(0.0);
    let hit = trace_ray(ray);
    if hit.hit {
        let sun_direction = normalize(vec4<f32>(0.3, -1.0, 0.2, 0.1));
        color = hit.color * max((dot(hit.normal, -sun_direction) * 0.5 + 0.5), 0.2);
    }
    textureStore(output_texture, coords, vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0));
}

struct Rotor {
    s: f32,
    e01: f32,
    e02: f32,
    e03: f32,
    e04: f32,
    e12: f32,
    e13: f32,
    e14: f32,
    e23: f32,
    e24: f32,
    e34: f32,
    e0123: f32,
    e0124: f32,
    e0134: f32,
    e0234: f32,
    e1234: f32,
}

fn transform(rotor: Rotor, point: vec4<f32>) -> vec4<f32> {
    let a = rotor.s;
    let b = rotor.e01;
    let c = rotor.e02;
    let d = rotor.e03;
    let f = rotor.e04;
    let g = rotor.e12;
    let h = rotor.e13;
    let i = rotor.e14;
    let j = rotor.e23;
    let k = rotor.e24;
    let l = rotor.e34;
    let m = rotor.e0123;
    let n = rotor.e0124;
    let o = rotor.e0134;
    let p = rotor.e0234;
    let q = rotor.e1234;
    let p0 = point.w;
    let p1 = point.z;
    let p2 = point.y;
    let p3 = point.x;
    let ap2 = a * p2;
    let gp3 = g * p3;
    let jp1 = j * p1;
    let kp0 = k * p0;
    let ap3 = a * p3;
    let gp2 = g * p2;
    let hp1 = h * p1;
    let ip0 = i * p0;
    let ap1 = a * p1;
    let lp0 = l * p0;
    let hp3 = h * p3;
    let jp2 = j * p2;
    let ap0 = a * p0;
    let lp1 = l * p1;
    let ip3 = i * p3;
    let kp2 = k * p2;
    let s0 = c + jp1 - ap2 - gp3 - kp0;
    let s1 = ap3 + b + hp1 - gp2 - ip0;
    let s2 = ap1 + d + jp2 - lp0 - hp3;
    let s3 = f + kp2 - ap0 - lp1 - ip3;
    return vec4<f32>(
        p0 + 2.0 * (q * (m + g * p1 + h * p2 + j * p3 - q * p0) + k * s0 + i * s1 + l * s2 - a * f - n * g - o * h - p * j),
        p1 + 2.0 * (a * d + m * g + q * (n + i * p2 + k * p3 - q * p1 - g * p0) + l * s3 - o * i - p * k - j * s0 - h * s1),
        p2 + 2.0 * (m * h + n * i + q * (l * p3 + o - q * p2 - h * p0 - i * p1) + g * s1 - a * c - l * p - k * s3 - j * s2),
        p3 + 2.0 * (a * b + l * o + m * j + n * k + q * (p - l * p2 - q * p3 - j * p0 - k * p1) + i * s3 + h * s2 + g * s0),
    ).wzyx;
}

fn transform_direction(rotor: Rotor, normal: vec4<f32>) -> vec4<f32> {
    let a = rotor.s;
    let f = rotor.e12;
    let g = rotor.e13;
    let h = rotor.e14;
    let i = rotor.e23;
    let j = rotor.e24;
    let k = rotor.e34;
    let p = rotor.e1234;
    let p0 = normal.w;
    let p1 = normal.z;
    let p2 = normal.y;
    let p3 = normal.x;
    let ap2 = a * p2;
    let fp3 = f * p3;
    let ip1 = i * p1;
    let jp0 = j * p0;
    let ap3 = a * p3;
    let fp2 = f * p2;
    let gp1 = g * p1;
    let hp0 = h * p0;
    let ap1 = a * p1;
    let kp0 = k * p0;
    let gp3 = g * p3;
    let ip2 = i * p2;
    let ap0 = a * p0;
    let kp1 = k * p1;
    let hp3 = h * p3;
    let jp2 = j * p2;
    let s0 = ip1 - ap2 - fp3 - jp0;
    let s1 = ap3 + gp1 - fp2 - hp0;
    let s2 = ap1 + ip2 - kp0 - gp3;
    let s3 = jp2 - ap0 - kp1 - hp3;
    return vec4<f32>(
        p0 + 2.0 * (p * (f * p1 + g * p2 + i * p3 - p * p0) + j * s0 + h * s1 + k * s2),
        p1 + 2.0 * (p * (h * p2 + j * p3 - p * p1 - f * p0) + k * s3 - i * s0 - g * s1),
        p2 + 2.0 * (p * (k * p3 - p * p2 - g * p0 - h * p1) + f * s1 - j * s3 - i * s2),
        p3 + 2.0 * (h * s3 + g * s2 + f * s0 - p * (k * p2 + p * p3 + i * p0 + j * p1)),
    ).wzyx;
}
