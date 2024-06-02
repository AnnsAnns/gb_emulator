use macroquad::prelude::*;

pub struct GbDisplay {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scaling: f32,
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
        //draw_text(
        //    "Game Display",
        //    self.offset_x + 100.0,
        //    self.offset_y + 250.0,
        //    100.0,
        //    BLACK,
        //);
    }

    pub fn size(&self, image: &Image) -> Vec2 {
        Vec2 {
            x: image.width() as f32 * self.scaling,
            y: image.height() as f32 * self.scaling,
        }
    }
}

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
                "Tile Index: {}, ADR: {:#X}",
                tile_index, tile_addr
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
        Vec2 {
            x: 8.0 * 16.0 * self.scaling,
            y: 8.0 * 24.0 * self.scaling + 20.0,
        }
    }
}

pub struct BackgroundViewer {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scaling: f32,
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
        Vec2 {
            x: (32.0 * 8.0) * self.scaling,
            y: (32.0 * 8.0) * self.scaling,
        }
    }
}

pub struct EmulationControls {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scaling: f32,

    play_active: Texture2D,
    play_inactive: Texture2D,
    pause_active: Texture2D,
    pause_inactive: Texture2D,
    step_active: Texture2D,
}

impl EmulationControls {
    pub fn new(offset_x: f32, offset_y: f32, scaling: f32) -> EmulationControls {
        let mut ec = EmulationControls {
            offset_x: offset_x,
            offset_y: offset_y,
            scaling: scaling,
            play_active: Texture2D::from_image(
                &Image::from_file_with_format(
                    include_bytes!("../../assets/buttons/play-active.png"),
                    Some(ImageFormat::Png),
                )
                .expect("Asset not found"),
            ),
            play_inactive: Texture2D::from_image(
                &Image::from_file_with_format(
                    include_bytes!("../../assets/buttons/play-inactive.png"),
                    Some(ImageFormat::Png),
                )
                .expect("Asset not found"),
            ),

            pause_active: Texture2D::from_image(
                &Image::from_file_with_format(
                    include_bytes!("../../assets/buttons/pause-active.png"),
                    Some(ImageFormat::Png),
                )
                .expect("Asset not found"),
            ),

            pause_inactive: Texture2D::from_image(
                &Image::from_file_with_format(
                    include_bytes!("../../assets/buttons/pause-inactive.png"),
                    Some(ImageFormat::Png),
                )
                .expect("Asset not found"),
            ),

            step_active: Texture2D::from_image(
                &Image::from_file_with_format(
                    include_bytes!("../../assets/buttons/step-active.png"),
                    Some(ImageFormat::Png),
                )
                .expect("Asset not found"),
            ),
        };

        ec.play_active.set_filter(FilterMode::Nearest);
        ec.play_inactive.set_filter(FilterMode::Nearest);
        ec.pause_active.set_filter(FilterMode::Nearest);
        ec.pause_inactive.set_filter(FilterMode::Nearest);
        ec.step_active.set_filter(FilterMode::Nearest);

        ec
    }

    pub fn draw(&self) {
        let button_params = DrawTextureParams {
            dest_size: Option::Some(Vec2::new(
                self.play_active.width() * self.scaling,
                self.play_active.height() * self.scaling,
            )),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            &self.play_active,
            self.offset_x,
            self.offset_y,
            WHITE,
            button_params.clone(),
        );
        draw_texture_ex(
            &self.pause_inactive,
            self.offset_x + self.play_active.width() * self.scaling + 5.0,
            self.offset_y,
            WHITE,
            button_params.clone(),
        );
        draw_texture_ex(
            &self.step_active,
            self.offset_x + self.play_active.width() * self.scaling * 2.0 + 30.0,
            self.offset_y,
            WHITE,
            button_params.clone(),
        );
    }
}
