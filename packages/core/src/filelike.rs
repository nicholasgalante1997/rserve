use core::fmt::Display;
use image;
use std::error::Error;
use std::fmt::{write, Debug};
use std::fs::{self, File};
use std::io::{Cursor, Read};

pub enum FileLike {
    TextFile(String),
    ImageFile(Vec<u8>),
}

pub enum KnownFileType {
    //Text Based Loading Strategy
    HTML,
    HTM,
    XHTML,
    XML,
    PHP,
    WASM,
    JS,
    CJS,
    MJS,
    JSON,
    JSON5,
    JSONLD,
    CSS,
    CSV,
    SVG,
    TEXT,

    // Image Based Loading Strategy
    AVIF,
    GIF,
    ICO,
    JPG,
    JPEG,
    PNG,
    WEBP,

    GZ,
    PDF,
    TAR,
    ZIP,
}

impl FileLike {
    pub fn get_file_type(path: &str) -> KnownFileType {
        let file_split_on_directory_delimiters: Vec<_> = path.split("/").collect();
        let last_index_of_file = file_split_on_directory_delimiters.len() - 1;
        let file_name = *file_split_on_directory_delimiters
            .get(last_index_of_file)
            .expect("Proved index is satisfactory length.");

        let file_name_split_on_dot_char: Vec<_> = file_name.split(".").collect();
        let last_index_of_file_name = file_name_split_on_dot_char.len() - 1;
        let file_ending = *file_name_split_on_dot_char
            .get(last_index_of_file_name)
            .expect("Proved index is satisfactory length.");

        if file_ending.to_lowercase() == "html" {
            return KnownFileType::HTML;
        }

        if file_ending.to_lowercase() == "htm" {
            return KnownFileType::HTM;
        }

        if file_ending.to_lowercase() == "xhtml" {
            return KnownFileType::XHTML;
        }

        if file_ending.to_lowercase() == "xml" {
            return KnownFileType::XML;
        }

        if file_ending.to_lowercase() == "php" {
            return KnownFileType::PHP;
        }

        if file_ending.to_lowercase() == "js" {
            return KnownFileType::JS;
        }

        if file_ending.to_lowercase() == "mjs" {
            return KnownFileType::MJS;
        }

        if file_ending.to_lowercase() == "cjs" {
            return KnownFileType::CJS;
        }

        if file_ending.to_lowercase() == "json" {
            return KnownFileType::JSON;
        }

        if file_ending.to_lowercase() == "json5" {
            return KnownFileType::JSON5;
        }

        if file_ending.to_lowercase() == "jsonld" {
            return KnownFileType::JSONLD;
        }

        if file_ending.to_lowercase() == "css" {
            return KnownFileType::CSS;
        }

        if file_ending.to_lowercase() == "csv" {
            return KnownFileType::CSV;
        }

        if file_ending.to_lowercase() == "svg" {
            return KnownFileType::SVG;
        }

        if file_ending.to_lowercase() == "txt" {
            return KnownFileType::TEXT;
        }

        if file_ending.to_lowercase() == "gif" {
            return KnownFileType::GIF;
        }

        if file_ending.to_lowercase() == "ico" {
            return KnownFileType::ICO;
        }

        if file_ending.to_lowercase() == "jpg" {
            return KnownFileType::JPG;
        }

        if file_ending.to_lowercase() == "jpeg" {
            return KnownFileType::JPEG;
        }

        if file_ending.to_lowercase() == "png" {
            return KnownFileType::PNG;
        }

        if file_ending.to_lowercase() == "webp" {
            return KnownFileType::WEBP;
        }

        if file_ending.to_lowercase() == "gz" {
            return KnownFileType::GZ;
        }

        if file_ending.to_lowercase() == "pdf" {
            return KnownFileType::PDF;
        }

        if file_ending.to_lowercase() == "tar" {
            return KnownFileType::TAR;
        }

        if file_ending.to_lowercase() == "zip" {
            return KnownFileType::ZIP;
        }

        KnownFileType::TEXT
    }

    pub fn use_text_file_loading_strategy(path: &str) -> Result<FileLike, Box<dyn Error>> {
        let file = fs::read_to_string(path)?;
        Ok(FileLike::TextFile(file))
    }

    pub fn use_image_file_loading_strategy(
        path: &str,
        output_format: image::ImageOutputFormat,
    ) -> Result<FileLike, Box<dyn Error>> {
        let file = image::open(path)?;
        let mut image_data: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        file.write_to(&mut image_data, output_format)?;
        Ok(FileLike::ImageFile(image_data.into_inner()))
    }

    pub fn get_filelike(path: &str) -> Result<FileLike, Box<dyn Error>> {
        let file_type = Self::get_file_type(path);
        match file_type {
            KnownFileType::HTML => Self::use_text_file_loading_strategy(path),
            KnownFileType::HTM => Self::use_text_file_loading_strategy(path),
            KnownFileType::XHTML => Self::use_text_file_loading_strategy(path),
            KnownFileType::XML => Self::use_text_file_loading_strategy(path),
            KnownFileType::PHP => Self::use_text_file_loading_strategy(path),
            KnownFileType::JS => Self::use_text_file_loading_strategy(path),
            KnownFileType::MJS => Self::use_text_file_loading_strategy(path),
            KnownFileType::CJS => Self::use_text_file_loading_strategy(path),
            KnownFileType::JSON => Self::use_text_file_loading_strategy(path),
            KnownFileType::JSON5 => Self::use_text_file_loading_strategy(path),
            KnownFileType::JSONLD => Self::use_text_file_loading_strategy(path),
            KnownFileType::CSS => Self::use_text_file_loading_strategy(path),
            KnownFileType::CSV => Self::use_text_file_loading_strategy(path),
            KnownFileType::SVG => Self::use_text_file_loading_strategy(path),

            KnownFileType::GIF => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::Gif)
            }
            KnownFileType::ICO => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::Ico)
            }
            KnownFileType::JPG => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::Jpeg(100))
            }
            KnownFileType::JPEG => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::Jpeg(100))
            }
            KnownFileType::PNG => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::Png)
            }
            KnownFileType::WEBP => {
                Self::use_image_file_loading_strategy(path, image::ImageOutputFormat::WebP)
            }

            _ => Self::use_text_file_loading_strategy(path),
        }
    }
}

impl FileLike {
    pub fn len(&self) -> usize {
        match self {
            FileLike::TextFile(file) => file.len(),
            FileLike::ImageFile(file) => file.len(),
        }
    }
}

impl Display for FileLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileLike::TextFile(file_as_string) => write(f, format_args!("{}", file_as_string)),
            FileLike::ImageFile(file_as_u8_vec) => write(f, format_args!("{:?}", file_as_u8_vec)),
        }
    }
}

impl Debug for FileLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileLike::TextFile(file_as_string) => write(f, format_args!("{}", file_as_string)),
            FileLike::ImageFile(file_as_u8_vec) => write(f, format_args!("{:?}", file_as_u8_vec)),
        }
    }
}
