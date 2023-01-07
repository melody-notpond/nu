use std::fmt::Display;

pub struct Buffers {
    buffers: Vec<Buffer>,
    current_buffer: usize,
}

impl Buffers {
    pub fn new(buffer: Buffer) -> Self {
        Buffers {
            buffers: vec![buffer],
            current_buffer: 0,
        }
    }

    pub fn add_buffer(&mut self, buffer: Buffer) -> usize {
        let id = self.buffers.len();
        self.buffers.push(buffer);
        id
    }

    pub fn get_current(&self) -> &Buffer {
        unsafe {
            self.buffers.get_unchecked(self.current_buffer)
        }
    }

    pub fn get_current_mut(&mut self) -> &mut Buffer {
        unsafe {
            self.buffers.get_unchecked_mut(self.current_buffer)
        }
    }

    pub fn modified(&self) -> Option<&Buffer> {
        self.buffers.iter().find(|v| v.modified)
    }

    pub fn remove_current(&mut self) {
        self.buffers.remove(self.current_buffer);
        if self.buffers.is_empty() {
            self.buffers.push(Buffer::new("[buffer]", false, ""));
        } else if self.current_buffer >= self.buffers.len() {
            self.current_buffer = self.buffers.len() - 1;
        }
    }

    pub fn next(&mut self) {
        self.current_buffer += 1;
        if self.current_buffer >= self.buffers.len() {
            self.current_buffer = 0;
        }
    }

    pub fn prev(&mut self) {
        if self.current_buffer == 0 {
            self.current_buffer = self.buffers.len();
        }
        self.current_buffer -= 1;
    }

    pub fn switch(&mut self, id: usize) {
        if id < self.buffers.len() {
            self.current_buffer = id;
        }
    }
}

pub struct Buffer {
    pub name: String,
    pub is_file: bool,
    pub modified: bool,
    pub vscroll: usize,
    update_vscroll: bool,
    pub hscroll: usize,
    update_hscroll: bool,

    pre: Vec<BufferLine>,
    post: Vec<BufferLine>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for line in self.pre.iter() {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "{}", line.pre)?;
            write!(f, "{}", line.post)?;
        }
        for line in self.post.iter() {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "{}", line.pre)?;
            write!(f, "{}", line.post)?;
        }

        Ok(())
    }
}

impl Buffer {
    pub fn new(name: &str, is_file: bool, contents: &str) -> Self {
        let mut post: Vec<_> = contents.split('\n').map(|v| BufferLine {
            pre: String::new(),
            post: v.to_owned(),
        }).collect();

        let pre = if post.is_empty() {
            vec![BufferLine { pre: String::new(), post: String::new(), }]
        } else {
            vec![post.remove(0)]
        };

        Buffer {
            name: name.to_owned(),
            is_file,
            modified: false,
            pre,
            post,
            vscroll: 0,
            update_vscroll: false,
            hscroll: 0,
            update_hscroll: false,
        }
    }

    pub fn window(&self, width: usize, height: usize) -> BufferWindow {
        BufferWindow {
            buffer: self,
            i: 0,
            width,
            height,
        }
    }

    pub fn move_left(&mut self) {
        if let Some(c) = self.pre.last_mut().unwrap().pre.pop() {
            self.pre.last_mut().unwrap().post.insert(0, c);
        } else if self.pre.len() > 1 {
            self.post.insert(0, self.pre.pop().unwrap());
            self.update_vscroll = true;
        }

        self.update_hscroll = true;
    }

    pub fn move_down(&mut self) {
        if !self.post.is_empty() {
            self.pre.push(self.post.remove(0));
            self.update_vscroll = true;
        }
    }

    pub fn move_up(&mut self) {
        if self.pre.len() > 1 {
            self.post.insert(0, self.pre.pop().unwrap());
            self.update_vscroll = true;
        }
    }

    pub fn move_right(&mut self) {
        if !self.pre.last().unwrap().post.is_empty() {
            let c = self.pre.last_mut().unwrap().post.remove(0);
            self.pre.last_mut().unwrap().pre.push(c);
        } else if !self.post.is_empty() {
            self.pre.push(self.post.remove(0));
            self.update_vscroll = true;
        }

        self.update_hscroll = true;
    }

    pub fn backspace(&mut self) {
        if self.pre.last_mut().unwrap().pre.pop().is_none()
            && self.pre.len() > 1
        {
            let last = self.pre.pop().unwrap();
            self.pre.last_mut().unwrap().post.push_str(&last.post);
            self.update_vscroll = true;
        }
        self.modified = true;
        self.update_hscroll = true;
    }

    pub fn enter(&mut self) {
        let mut post = String::new();
        std::mem::swap(&mut post, &mut self.pre.last_mut().unwrap().post);
        self.pre.push(BufferLine { pre: String::new(), post, });
        self.update_vscroll = true;
        self.modified = true;
    }

    pub fn char(&mut self, c: char) {
        self.pre.last_mut().unwrap().pre.push(c);
        self.update_vscroll = true;
        self.update_hscroll = true;
        self.modified = true;
    }

    pub fn update_scrolls(&mut self, width: isize, height: isize) {
        if self.update_vscroll {
            self.update_vscroll = false;

            if self.pre.len() as isize - (self.vscroll as isize) > height {
                self.vscroll = self.pre.len() - height as usize;
            } else if self.pre.len() as isize - (self.vscroll as isize) <= 0 {
                self.vscroll = self.pre.len() - 1;
            }
        }

        if self.update_hscroll {
            self.update_hscroll = false;

            let v = self.pre.last().unwrap().pre.len();
            if v as isize - (self.hscroll as isize) > width - 1 {
                self.hscroll = v - width as usize + 1;
            } else if v as isize - (self.hscroll as isize) <= 0 {
                self.hscroll = v;
            }
        }
    }

    pub fn cursor_pos(&self, x: usize, y: usize) -> (usize, usize) {
        (x + self.pre.last().unwrap().pre.len() - self.hscroll, y as usize + self.pre.len() - self.vscroll - 1)
    }

    pub fn line_count(&self) -> usize {
        self.pre.len() + self.post.len()
    }
}

struct BufferLine {
    pre: String,
    post: String,
}


pub struct BufferWindow<'a> {
    buffer: &'a Buffer,
    i: usize,
    width: usize,
    height: usize,
}

impl<'a> Iterator for BufferWindow<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.height {
            let i = self.i + self.buffer.vscroll;
            self.i += 1;
            let line = if let Some(v) = self.buffer.pre.get(i) {
                v
            } else if let Some(v) = self.buffer.post.get(i - self.buffer.pre.len()) {
                v
            } else {
                return None;
            };
            let v = vec![line.pre.as_str(), line.post.as_str()];

            let mut j = 0;
            Some(v.into_iter().filter_map(|v| {
                let bound_left = j <= self.buffer.hscroll && self.buffer.hscroll < j + v.len();
                let bound_right = j <= self.buffer.hscroll + self.width && self.buffer.hscroll + self.width < j + v.len();
                let u = if !bound_left && !bound_right {
                    if self.buffer.hscroll < j && j + v.len() < self.buffer.hscroll + self.width {
                        Some(v)
                    } else {
                        None
                    }
                } else if !bound_left && bound_right {
                    Some(&v[..self.buffer.hscroll + self.width - j])
                } else if bound_left && !bound_right {
                    Some(&v[self.buffer.hscroll - j..])
                } else {
                    Some(&v[self.buffer.hscroll - j..self.buffer.hscroll + self.width - j])
                };
                j += v.len();
                u
            }).collect())
        } else {
            None
        }
    }
}
