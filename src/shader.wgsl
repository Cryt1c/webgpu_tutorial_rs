// Vertex shader

struct CameraUniform {
  view_proj: mat4x4<f32>,
  position: vec3<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.position + vec3(0.5);
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var texture: texture_3d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

const MAX_SAMPLES: i32 = 2000;
const MIN_TEX: vec3<f32> = vec3(0.0);
const MAX_TEX: vec3<f32> = vec3(1.0);
const STEP_SIZE: f32 = 0.001;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var data_position: vec3<f32> = in.tex_coords;
    var direction: vec3<f32> = normalize((in.tex_coords - vec3(0.5)) - camera.position);
    var step: vec3<f32> = direction * STEP_SIZE;

    var max_value: f32 = 0.0;
    var stop: bool = false;

    var result = vec4(0.0, 0.0, 0.0, 0.0);

    for (var i: i32 = 0; i < MAX_SAMPLES; i++) {
        data_position += step;
        stop = dot(sign(data_position - MIN_TEX), sign(MAX_TEX - data_position)) < 3.0;

        if stop {
            break;
        }

        var value: f32 = textureSample(texture, texture_sampler, data_position).r;
        if value > max_value {
            max_value = value;
        }
    }
    return vec4(max_value, max_value, max_value, max_value);
}
