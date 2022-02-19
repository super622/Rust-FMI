use ggez::*;

pub struct Assets{
    //black jet
    pub black_jet_img: graphics::Image,
    //white jet
    pub white_jet_img: graphics::Image,
    pub shot_img: graphics::Image,
    pub shooting_sound: audio::Source,
    pub hitted_sound: audio::Source,
    pub font: graphics::Font,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let black_jet_image = graphics::Image::new(ctx,"/black-jet.png")?;
        let white_jet_image = graphics::Image::new(ctx,"/white-jet.png")?;
        let shot_image = graphics::Image::new(ctx,"/shot.png")?;
        
        let shooting_sound = audio::Source::new(ctx,"/shooting.ogg")?;
        let hitted_sound = audio::Source::new(ctx, "/boom.ogg")?;

        let font = graphics::Font::new(ctx,"/font.ttf")?; 

        let asset = Assets{
            black_jet_img: black_jet_image,
            white_jet_img: white_jet_image,
            shot_img: shot_image,
            shooting_sound: shooting_sound,
            hitted_sound: hitted_sound,
            font: font
        };

        Ok(asset)
    }

    pub fn white_jet_image(&mut self,) -> &mut graphics::Image{
        &mut self.white_jet_img
    }

    pub fn black_jet_image(&mut self) -> &mut graphics::Image{
        &mut self.black_jet_img
    }

    pub fn shot_image(&mut self) -> &mut graphics::Image{
        &mut self.shot_img
    }
}