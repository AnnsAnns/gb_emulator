use macroquad::prelude::*;

pub struct TileViewer {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scaling: f32,
}

impl TileViewer {
    pub fn draw(&mut self, atlas: &Image) {
        let tex2d_params = DrawTextureParams {
            dest_size: Option::Some(Vec2::new(
                atlas.width() as f32 * self.scaling,
                atlas.height() as f32 * self.scaling,
            )),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
    
        let tex2d = Texture2D::from_image(atlas);
        tex2d.set_filter(FilterMode::Nearest);
        draw_texture_ex(&tex2d, self.offset_x, self.offset_y, WHITE, tex2d_params);
    
        let mouse_pos = mouse_position();
    
        if mouse_pos.0 >= self.offset_x
            && mouse_pos.0 < self.offset_x + 16.0 * 8.0 * self.scaling
            && mouse_pos.1 >= self.offset_y
            && mouse_pos.1 < self.offset_y + 24.0 * 8.0 * self.scaling
        {
            let x_tile = ((mouse_pos.0 - self.offset_x) / self.scaling).floor() as u32 / 8;
            let y_tile = ((mouse_pos.1 - self.offset_y) / self.scaling).floor() as u32 / 8;
            let tile_index = y_tile * 16 + x_tile;
            let tile_addr = 0x8000 + tile_index * 16;
    
            let pos_text = format!(
                "Tile Index: {}, Tile X: {}, Tile Y: {}, ADR: {:#X}",
                tile_index, x_tile, y_tile, tile_addr
            );
    
            draw_text(
                &pos_text,
                self.offset_x + 4.0,
                self.offset_y + 24.0 * 8.0 * self.scaling + 16.0,
                16.0,
                WHITE,
            );
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {x: 8.0 * 16.0 * self.scaling, y: 8.0 * 24.0 * self.scaling + 15.0}
    }

}

pub struct GbDisplay {
    pub offset_x: f32, 
    pub offset_y: f32,
    pub scaling: f32
}

impl GbDisplay {
    pub fn draw(&mut self, image: &Image) {
        let tex2d_params = DrawTextureParams {
            dest_size: Option::Some(Vec2::new(
                image.width() as f32 * self.scaling,
                image.height() as f32 * self.scaling,
            )),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
    
        let tex2d = Texture2D::from_image(image);
        tex2d.set_filter(FilterMode::Nearest);
        draw_texture_ex(&tex2d, self.offset_x, self.offset_y, WHITE, tex2d_params);
    
        //TODO: Draw actual emulator content
        draw_text(
            "Game Display",
            self.offset_x + 100.0,
            self.offset_y + 250.0,
            100.0,
            BLACK,
        );
    }

    pub fn size(&self, image: &Image) -> Vec2 {
        Vec2 {x: image.width() as f32 * self.scaling, y: image.height() as f32 * self.scaling}
    }

}

pub struct BackgroundViewer {
    pub offset_x: f32, 
    pub offset_y: f32, 
    pub scaling: f32
}

impl BackgroundViewer {
    pub fn draw(&mut self, image: &Image) {
        let tex2d_params = DrawTextureParams {
            dest_size: Option::Some(Vec2::new(
                image.width() as f32 * self.scaling,
                image.height() as f32 * self.scaling,
            )),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
    
        let tex2d = Texture2D::from_image(image);
        tex2d.set_filter(FilterMode::Nearest);
        draw_texture_ex(&tex2d, self.offset_x, self.offset_y, WHITE, tex2d_params);
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {x: (32.0 * 8.0) * self.scaling, y: (32.0 * 8.0) * self.scaling}
    }
}