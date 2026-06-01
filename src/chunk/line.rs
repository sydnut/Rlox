#[derive(Debug)]
struct Line {
    line: u32,
    count: u32,
}
impl Line {
    fn new(line: u32) -> Self {
        Self { line, count: 1 }
    }
    fn add_count(&mut self) {
        self.count += 1;
    }
}
#[derive(Debug)]
pub struct Lines {
    lines: Vec<Line>,
}
impl Lines {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }
    pub fn add_line(&mut self, line: u32) {
        for x in &mut self.lines {
            if x.line < line {
                continue;
            } else if x.line == line {
                x.add_count();
                return;
            }
        }
        //都没有,新增
        self.lines.push(Line::new(line));
    }
    ///
    /// 返回字节码索引的行号
    pub fn get_line(&self, idx: u32) -> Result<u32, String> {
        let mut cnt = 0;
        for line in self.lines.iter() {
            cnt += line.count;
            if cnt < idx {
                continue;
            } else {
                return Ok(line.line);
            }
        }
        Err(format!(
            "Line index {} out of bounds! Not found the bytecode",
            idx
        ))
    }
}
