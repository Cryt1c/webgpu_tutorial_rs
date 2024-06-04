use anyhow::*;
use three_d_asset::{Texture3D, TextureData};

#[derive(Debug, PartialEq)]
pub struct Dim {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

pub struct VolTexture {
    pub texture_data: Vec<u8>,
    pub dimensions: Dim,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vol_texture: &VolTexture,
        label: &str,
    ) -> Result<Self> {
        let rgba = &vol_texture.texture_data;

        let size = wgpu::Extent3d {
            width: vol_texture.dimensions.width as u32,
            height: vol_texture.dimensions.height as u32,
            depth_or_array_layers: vol_texture.dimensions.depth as u32,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            // Tells wgpu where to copy the pixel data,
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(vol_texture.dimensions.width as u32),
                rows_per_image: Some(vol_texture.dimensions.height as u32),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    pub fn read_vol(file_path: &str) -> VolTexture {
        let texture_3d: Texture3D = three_d_asset::io::load(&[file_path])
            .unwrap()
            .deserialize("")
            .unwrap();
        let width = texture_3d.width as i32;
        let height = texture_3d.height as i32;
        let depth = texture_3d.depth as i32;
        let texture_data = match texture_3d.data {
            TextureData::RU8(data) => data,
            _ => panic!("Expected RU8 texture data format"),
        };

        VolTexture {
            dimensions: Dim {
                width,
                height,
                depth,
            },
            texture_data,
        }
    }
}
