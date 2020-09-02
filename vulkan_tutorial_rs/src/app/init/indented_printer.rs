use std::fmt::Debug;

pub struct IndentedPrinter {
    indent: String,
    indent_step: String,
}

impl IndentedPrinter {
    pub fn _new(indent_step: &str) -> IndentedPrinter {
        IndentedPrinter {
            indent: String::new(),
            indent_step: String::from(indent_step),
        }
    }

    pub fn new_with_base(indent_base: &str, indent_step: &str) -> IndentedPrinter {
        IndentedPrinter {
            indent: String::from(indent_base),
            indent_step: String::from(indent_step),
        }
    }

    pub fn indent(&mut self) {
        self.indent.push_str(&self.indent_step);
    }

    pub fn _unindent(&mut self) {
        self.indent
            .truncate(self.indent.len() - self.indent_step.len());
    }

    pub fn print_line(&self, line: &str) {
        println!("{indent}{line}", indent = self.indent, line = line);
    }

    pub fn print_key_value(&self, key: &str, value: &str) {
        println!("{indent}{key}:", indent = self.indent, key = key);
        println!(
            "{indent}{indent_step}{value}",
            indent = self.indent,
            indent_step = self.indent_step,
            value = value
        );
    }

    pub fn print_key_value_debug<Val: Debug>(&self, key: &str, value: &Val) {
        self.print_key_value(key, &format!("{:?}", value));
    }
}
