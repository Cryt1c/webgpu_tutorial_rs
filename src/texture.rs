use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::{
    fs::{read_to_string, File},
    io::BufReader,
};

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

    pub fn parse_meta_data_dim(meta_data: &str) -> Dim {
        let mut dimensions = Dim {
            width: 0,
            height: 0,
            depth: 0,
        };

        meta_data.lines().for_each(|line| {
            let mut data = line.split(" = ");

            match data.next() {
                Some("NDims") => {
                    let _value = data.next().unwrap().parse::<i32>().unwrap();
                    // TODO: Use NDims info.
                }
                Some("DimSize") => {
                    let value = data
                        .next()
                        .unwrap()
                        .split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    dimensions.width = value[0];
                    dimensions.height = value[1];
                    dimensions.depth = value[2];
                }
                Some("ElementSpacing") => {
                    let _value = data.next().unwrap();
                    // TODO: Use ElementSpacing info.
                }
                _ => {
                    println!("Unknown field");
                }
            }
        });

        dimensions
    }

    pub fn read_raw(file_path: &str, meta_file_path: &str) -> VolTexture {
        let meta_data = read_to_string(meta_file_path).expect("Unable to read MHD file");
        let dimensions = Texture::parse_meta_data_dim(&meta_data);

        let num_elements = dimensions.height * dimensions.width * dimensions.depth;
        let file = File::open(file_path).expect("Unable to open RAW file");
        let mut reader = BufReader::new(file);

        let mut raw_data = vec![0u16; num_elements as usize];
        reader
            .read_u16_into::<LittleEndian>(&mut raw_data)
            .expect("Unable to read u16 from RAW file");

        let buffer: Vec<u8> = raw_data
            .iter()
            .map(|&value| Texture::normalize_hounsfield_units(value))
            .collect();

        VolTexture {
            dimensions: Dim {
                width: dimensions.width,
                height: dimensions.height,
                depth: dimensions.depth,
            },
            texture_data: buffer,
        }
    }

    pub fn normalize_hounsfield_units(value: u16) -> u8 {
        let normalized_hu_value = (value as f32 / 4095.0) * 255.0; // Normalize to [0, 255]
        normalized_hu_value as u8
    }
}
