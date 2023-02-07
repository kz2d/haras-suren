// use image::{codecs::jpeg::JpegEncoder, ColorType};
// use usvg::{Tree, Image};
// use resvg::render;

// fn render_svg(svg: &str, dpi: Option<usize>, alpha: bool) -> Result<Image, SvgError> {
//     let mut options = usvg::Options::default();
//     if let Some(dpi) = dpi {
//         options.dpi = dpi as f64;
//     }
//     let tree = Tree::from_str(svg, &options)?;
//     let fit_to = usvg::FitTo::Original;
//     let background = if alpha {
//         None
//     } else {
//         Some(svgtypes::Color::white())
//     };
//     render(&tree, fit_to, background).ok_or(SvgError::Unspecified)
// }

// /// Convert a svg str to a jpg file
// pub fn convert_svg_to_jpg(svg: &str, dpi: Option<usize>) -> Result<Vec<u8>, SvgError> {
//     let image = render_svg(svg, dpi, false)?;
//     let width = image.width();
//     let height = image.height();

//     let mut jpeg = Vec::new();
//     let mut encoder = JpegEncoder::new(&mut jpeg);
//     encoder.encode(image.data(), width, height, ColorType::Rgb8)?;
//     Ok(jpeg)
// }


// #[derive(Debug)]
// pub enum SvgError {
//     Unspecified,
//     Usvg(usvg::Error),
//     Image(image::error::ImageError),
//     Io(std::io::Error),
// }

// impl Error for SvgError {}

// impl fmt::Display for SvgError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl From<usvg::Error> for SvgError {
//     fn from(src: usvg::Error) -> Self {
//         SvgError::Usvg(src)
//     }
// }

// impl From<image::error::ImageError> for SvgError {
//     fn from(src: image::error::ImageError) -> Self {
//         SvgError::Image(src)
//     }
// }

// impl From<std::io::Error> for SvgError {
//     fn from(src: std::io::Error) -> Self {
//         SvgError::Io(src)
//     }
// }