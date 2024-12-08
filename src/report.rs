pub struct Reporter {
    data: Vec<ReportData>,
    had_error: bool,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            had_error: false,
            data: Vec::new(),
        }
    }

    pub fn report(&mut self, line: usize, message: String) {
        self.data.push(ReportData::new(line, message));
        self.had_error = true;
    }

    pub fn report_with_local(&mut self, line: usize, message: String, local: String) {
        self.data
            .push(ReportData::new_with_local(line, message, local));
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn print(&self) {
        for input in &self.data {
            let local = input.local.as_deref().unwrap_or_default();
            eprintln!("[line {}] Error{}: {}", input.line, local, input.message);
        }
    }
}

pub struct ReportData {
    line: usize,
    message: String,
    local: Option<String>,
}

impl ReportData {
    pub fn new(line: usize, message: String) -> Self {
        Self {
            line,
            message,
            local: None,
        }
    }

    fn new_with_local(line: usize, message: String, local: String) -> Self {
        Self {
            line,
            message,
            local: Some(local),
        }
    }
}
