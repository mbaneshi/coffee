use gfx_device_gl as gl;
use gfx_glyph::GlyphCruncher;

use crate::graphics::gpu::{TargetView, Transformation};
use crate::graphics::Text;

pub struct Font {
    glyphs: gfx_glyph::GlyphBrush<'static, gl::Resources, gl::Factory>,
}

impl Font {
    pub fn from_bytes(factory: &mut gl::Factory, bytes: &'static [u8]) -> Font {
        Font {
            glyphs: gfx_glyph::GlyphBrushBuilder::using_font_bytes(bytes)
                .depth_test(gfx::preset::depth::PASS_TEST)
                .texture_filter_method(gfx::texture::FilterMethod::Scale)
                .build(factory.clone()),
        }
    }

    pub fn add(&mut self, text: Text) {
        self.glyphs.queue(gfx_glyph::Section {
            text: &text.content,
            screen_position: (text.position.x, text.position.y),
            scale: gfx_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into_linear(),
            bounds: text.bounds,
            ..Default::default()
        });
    }

    pub fn measure(&mut self, text: Text) -> (f32, f32) {
        let bounds = self.glyphs.pixel_bounds(gfx_glyph::Section {
            text: &text.content,
            screen_position: (text.position.x, text.position.y),
            scale: gfx_glyph::Scale {
                x: text.size,
                y: text.size,
            },
            color: text.color.into_linear(),
            bounds: text.bounds,
            ..Default::default()
        });

        match bounds {
            Some(bounds) => (bounds.width() as f32, bounds.height() as f32),
            None => (0.0, 0.0),
        }
    }

    pub fn draw(
        &mut self,
        encoder: &mut gfx::Encoder<gl::Resources, gl::CommandBuffer>,
        target: &TargetView,
        transformation: Transformation,
    ) {
        let typed_target: gfx::handle::RenderTargetView<
            gl::Resources,
            gfx::format::Srgba8,
        > = gfx::memory::Typed::new(target.clone());

        self.glyphs
            .use_queue()
            .transform(transformation)
            .draw(encoder, &typed_target)
            .expect("Font draw");
    }
}
