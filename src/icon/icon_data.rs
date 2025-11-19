use std::sync::LazyLock;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct IconData {
    pub class: Option<&'static str>,
    pub x: Option<&'static str>,
    pub y: Option<&'static str>,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
    pub view_box: Option<&'static str>,
    pub stroke_linecap: Option<&'static str>,
    pub stroke_linejoin: Option<&'static str>,
    pub stroke_width: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub data: &'static str,
}

// Setters
impl IconData {
    pub fn set_class(mut self, class: Option<&'static str>) -> Self {
        self.class = class;
        self
    }
    pub fn set_x(mut self, x: Option<&'static str>) -> Self {
        self.x = x;
        self
    }
    pub fn set_y(mut self, y: Option<&'static str>) -> Self {
        self.y = y;
        self
    }
    pub fn set_width(mut self, width: Option<&'static str>) -> Self {
        self.width = width;
        self
    }
    pub fn set_height(mut self, height: Option<&'static str>) -> Self {
        self.height = height;
        self
    }
    pub fn set_view_box(mut self, view_box: Option<&'static str>) -> Self {
        self.view_box = view_box;
        self
    }
    pub fn set_stroke_linecap(mut self, stroke_linecap: Option<&'static str>) -> Self {
        self.stroke_linecap = stroke_linecap;
        self
    }
    pub fn set_stroke_linejoin(mut self, stroke_linejoin: Option<&'static str>) -> Self {
        self.stroke_linejoin = stroke_linejoin;
        self
    }
    pub fn set_stroke_width(mut self, stroke_width: Option<&'static str>) -> Self {
        self.stroke_width = stroke_width;
        self
    }
    pub fn set_stroke(mut self, stroke: Option<&'static str>) -> Self {
        self.stroke = stroke;
        self
    }
    pub fn set_fill(mut self, fill: Option<&'static str>) -> Self {
        self.fill = fill;
        self
    }
    pub fn set_data(mut self, data: &'static str) -> Self {
        self.data = data;
        self
    }
    pub fn filled(mut self) -> Self{
        self.fill = Some("currentColor");
        self
    }
}

#[macro_export] 
/// Signature 1: `icon_data!(path: LazyLock<String>, width: u32, height: u32)` \
/// Signature 2: `icon_data!(path: LazyLock<String>, side: u32)` \
/// This is not a simple constructor because icons need static str refs to their width, height, viewbox etc.
macro_rules! icon_data {
    ($html:expr, $side:expr) => {
        icon_data!($html, $side, $side)
    };
    ($html:expr, $width:expr, $height:expr) => {
        IconData {
            class: None,
            x: Some("0"),
            y: Some("0"),
            width: Some(stringify!($width)),
            height: Some(stringify!($height)),
            view_box: Some(concat!(
                "0 0 ",
                stringify!($width),
                " ",
                stringify!($height)
            )),
            stroke_linecap: None,
            stroke_linejoin: None,
            stroke_width: None,
            stroke: None,
            fill: None,
            data: $html,
        }
    };
}

pub type IconRef = &'static IconData;
