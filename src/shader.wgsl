@group(0)
@binding(0)
var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Camera {
    v_fov: f32,
}

@group(1)
@binding(0)
var<uniform> camera: Camera;

struct Ray {
    origin: vec4<f32>,
    direction: vec4<f32>,
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

fn trace_ray(ray: Ray) -> vec3<f32> {
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
        let normal = -step_axis * step_dir;

        if all(voxel_pos >= vec4<i32>(0)) && all(voxel_pos < vec4<i32>(4)) {
            return chunk.data[u32(voxel_pos.x) + u32(voxel_pos.y) * 4 + u32(voxel_pos.z) * 4 * 4 + u32(voxel_pos.w) * 4 * 4 * 4].color;
        }
    }

    return vec3<f32>(0.0, 0.0, 1.0);
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
    ray.origin = vec4<f32>(-4.5, 0.5, -1.5, 0.5);
    ray.direction = normalize(vec4<f32>(1.0, normalized_uv.y * theta, normalized_uv.x * aspect * theta, 0.0));

    let color = trace_ray(ray);
    textureStore(output_texture, coords, vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0));
}
