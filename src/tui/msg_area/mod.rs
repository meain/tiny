pub mod line;

use std::cmp::{max, min};
use std::mem;
use std::str;

use rustbox::{RustBox};

use self::line::Line;
use tui::style::StyleRef;

pub struct MsgArea {
    lines       : Vec<Line>,

    // Rendering related
    width       : i32,
    height      : i32,

    /// Vertical scroll: An offset from the last visible line.
    /// E.g. when this is 0, `self.lines[self.lines.len() - 1]` is drawn at the
    /// bottom of screen.
    scroll      : i32,

    line_buf    : Line,
}

impl MsgArea {
    pub fn new(width : i32, height : i32) -> MsgArea {
        MsgArea {
            lines: Vec::new(),
            width: width,
            height: height,
            scroll: 0,
            line_buf: Line::new(),
        }
    }

    pub fn resize(&mut self, width : i32, height : i32) {
        self.width = width;
        self.height = height;
    }

    pub fn draw(&self, rustbox : &RustBox, pos_x : i32, pos_y : i32) {
        let mut row = pos_y + self.height - 1;

        // Draw lines in reverse order
        let mut line_idx = ((self.lines.len() as i32) - 1) - self.scroll;
        while line_idx > 0 && row >= pos_y {
            let line = unsafe { self.lines.get_unchecked(line_idx as usize) };

            // Where to start rendering this line?
            let line_height = line.rendered_height(self.width);
            let line_row = row - line_height + 1;

            // Do we have enough space to render this line?
            if line_row < pos_y {
                break;
            }

            // Render it
            line.draw(rustbox, pos_x, line_row, self.width);

            row = line_row - 1;
            line_idx -= 1;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Scrolling

impl MsgArea {
    pub fn scroll_up(&mut self) {
        if self.scroll < (self.lines.len() as i32) - 1 {
            self.scroll += 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn page_up(&mut self) {
        self.scroll = max(0, min((self.lines.len() as i32) - 1, self.scroll + 10));
    }

    pub fn page_down(&mut self) {
        self.scroll = max(0, self.scroll - 10);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Adding text

impl MsgArea {
    pub fn add_text(&mut self, str : &str, style : StyleRef) {
        self.line_buf.add_text(str, style);
    }

    pub fn flush_line(&mut self) {
        self.lines.push(mem::replace(&mut self.line_buf, Line::new()));
    }
}