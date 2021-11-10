use std::hash::Hasher;

use crate::parser::{Instruction, Method};

struct Gen {
    builder: String,
    method_gen: MethodGen,
}

impl Gen {
    const fn new() -> Self {
        Self {
            builder: String::new(),
            method_gen: MethodGen::new(),
        }
    }

    fn write(&mut self, content: &str) {
        if self.method_gen.empty_line {
            self.builder
                .push_str(&" ".repeat((self.method_gen.indent * 4) as usize));
            self.method_gen.empty_line = !self.method_gen.empty_line;
        }

        self.builder.push_str(content);

        if content.ends_with("\n") || content.ends_with("\r") {
            self.method_gen.empty_line = !self.method_gen.empty_line;
        }
    }

    fn writeln(&mut self, content: &str) {
        if self.method_gen.empty_line {
            self.builder
                .push_str(&" ".repeat((self.method_gen.indent * 4) as usize));
            self.method_gen.empty_line = !self.method_gen.empty_line;
        }

        self.builder.push_str(content);
        self.builder.push_str("\n");
        self.method_gen.empty_line = !self.method_gen.empty_line;
    }
}

struct MethodGen {
    indent: u32,
    empty_line: bool,
    tmp_variable: u32,
}

impl MethodGen {
    const fn new() -> Self {
        Self {
            indent: 0,
            empty_line: true,
            tmp_variable: 0,
        }
    }

    fn inc_indent(&mut self) {
        self.indent += 1;
    }

    fn dec_indent(&mut self) {
        self.indent -= 1;
    }

    fn next_tmp(&mut self) -> String {
        self.tmp_variable += 1;
        format!("%{}", self.tmp_variable)
    }
}

pub fn gen(methods: Vec<Method>) -> String {
    let mut gen = Gen::new();
    let mut method_iter = methods.iter().peekable();

    while method_iter.peek().is_some() {
        gen_method(&mut gen, method_iter.next().unwrap());
        gen.method_gen = MethodGen::new();
    }

    gen.builder
}

fn gen_method(gen: &mut Gen, method: &Method) {
    // method header
    gen.write("define ");
    gen.write(&method.return_type);
    gen.write(" @");
    gen.write(&method.name);
    gen.write("(");
    gen.writeln(") {");
    gen.method_gen.inc_indent();

    // method body
    let mut instruction_iter = method.instructions.iter().peekable();

    while instruction_iter.peek().is_some() {
        match &instruction_iter.next().unwrap() {
            Instruction::Push(token) => {
                let temp = &gen.method_gen.next_tmp();
                gen.write(temp);
                gen.writeln(" = alloca i32");
                gen.write("store i32 ");
                gen.write(&token.literal);
                gen.write(", i32* ");
                gen.writeln(temp);
            }
            Instruction::Ret() => gen.writeln("ret void"),
            _ => break,
        }
    }

    // method tail
    gen.method_gen.dec_indent();
    gen.writeln("}")
}
