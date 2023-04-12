use tui::{
    style::Style,
    text::{Spans, Text},
};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Choice<T> {
    key: T,
    text: T,
    bracket_style: Style,
}

impl<T> Choice<T> {
    pub fn new(key: T, text: T, bracket_style: Style) -> Self {
        Self {
            key,
            text,
            bracket_style,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Section<T> {
    title: Option<T>,
    choices: Vec<Choice<T>>,
}

impl<T> Section<T> {
    pub fn new(title: Option<T>, choices: Vec<Choice<T>>) -> Self {
        Self { title, choices }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TuiDashboardWidget<'a, T: Into<Text<'a>>> {
    text: Vec<Spans<'a>>,
    sections: Vec<Section<T>>,
}

// https://github.com/gin66/tui-logger/blob/master/src/lib.rs#L1120
// impl Widget for TuiDashboardWidget<>
