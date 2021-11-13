use rayon::vec;

use crate::{
    lexer::Type,
    parser::{Instruction, Method},
};

struct Gen {
    builder: String,
    method_gen: MethodGen,
}

macro_rules! write {
    ($gen:expr, $($arg:tt)*) => {
        let result = format!($($arg)*);
        $gen.write(&result);
    };
}

macro_rules! writeln {
    ($gen:expr, $($arg:tt)*) => {
        $gen.writeln(&format!($($arg)*));
    };
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
    stack_tracker: Vec<StackItem>,
    indent: u32,
    empty_line: bool,
    variable_index: usize,
}

struct StackItem {
    index: usize,
    item_type: String,
    is_ptr: bool,
}

impl StackItem {
    const fn new(index: usize, item_type: String, is_ptr: bool) -> Self {
        Self {
            index,
            item_type,
            is_ptr,
        }
    }
}

impl MethodGen {
    const fn new() -> Self {
        Self {
            stack_tracker: vec![],
            indent: 0,
            empty_line: true,
            variable_index: 0,
        }
    }

    fn inc_indent(&mut self) {
        self.indent += 1;
    }

    fn dec_indent(&mut self) {
        self.indent -= 1;
    }

    fn allocate_variable(&mut self, type_name: &String, is_ptr: bool, count: usize) -> Vec<String> {
        let variables = (self.variable_index + 1..=self.variable_index + count)
            .map(|i| format!("%{}", i))
            .collect::<Vec<String>>();
        self.variable_index += count;
        self.stack_tracker.push(StackItem::new(
            self.variable_index,
            type_name.to_string(),
            is_ptr,
        ));
        variables
    }

    fn consume_variable(&mut self, count: usize) -> Vec<StackItem> {
        let mut variables = vec![];

        if self.stack_tracker.len() < count {
            return variables;
        }

        for _ in 0..count {
            variables.push(self.stack_tracker.pop().unwrap());
        }

        variables
    }

    fn assert_same_type(&self, expected_type: &String, operand_count: usize) -> bool {
        if self.stack_tracker.len() < operand_count {
            return false;
        }

        let index = self.stack_tracker.len() - operand_count;
        let mut stack_items = (&self.stack_tracker[index..]).iter();

        if operand_count == 1 {
            stack_items.next().unwrap().item_type == *expected_type
        } else {
            stack_items.all(|item| item.item_type == *expected_type)
        }
    }

    fn last_type(&self) -> String {
        return self
            .stack_tracker
            .last()
            .expect("Expected operand")
            .item_type
            .clone();
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
    writeln!(gen, "define {} @{}() {{", method.return_type, method.name);
    gen.method_gen.inc_indent();

    // method body
    let mut instruction_iter = method.instructions.iter().peekable();
    let mut last_instruction = method
        .instructions
        .first()
        .expect("Expected instruction but found nothing");

    while instruction_iter.peek().is_some() {
        let instruction = &instruction_iter.next().unwrap();

        match instruction {
            Instruction::Push(token) => {
                let value_type = match token.token_type {
                    Type::Integer => "i32",
                    _ => "Expected value",
                };
                let variable = gen
                    .method_gen
                    .allocate_variable(&value_type.to_string(), true, 1);
                writeln!(gen, "{0} = alloca {1}", variable[0], value_type);
                writeln!(
                    gen,
                    "store {0} {1}, {0}* {2}",
                    value_type, token.literal, variable[0]
                );
            }
            Instruction::Add()
            | Instruction::Sub()
            | Instruction::Mul()
            | Instruction::Div()
            | Instruction::Rem() => {
                let operand_type = gen.method_gen.last_type();

                if !gen.method_gen.assert_same_type(&operand_type, 2) {
                    panic!("Types of operands must be same");
                }

                // let ptr_count = gen.method_gen.stack_tracker
                //     [gen.method_gen.stack_tracker.len() - 2..]
                //     .iter()
                //     .map(|operand| if operand.is_ptr { 1 } else { 0 })
                //     .sum();

                // let operands = &gen.method_gen.consume_variable(2);

                // let tmp_variable =
                //     gen.method_gen
                //         .allocate_variable(&operand_type, false, ptr_count);

                // for i in 0..ptr_count {
                //     writeln!(
                //         gen,
                //         "{0} = load {1}, {1}* %{2}",
                //         tmp_variable[i],
                //         operand_type,
                //         gen.method_gen.variable_index - 4 + i
                //     )
                // }

                let mut operands = Vec::<StackItem>::new();

                for _ in 0..2 {
                    let stack_item = gen.method_gen.stack_tracker.pop().unwrap();

                    if stack_item.is_ptr {
                        let tmp_variable = gen.method_gen.allocate_variable(&operand_type, false, 1);

                        writeln!(
                            gen,
                            "{0} = load {1}, {1}* %{2}",
                            tmp_variable[0],
                            &operand_type,
                            stack_item.index
                        );

                        operands.push(gen.method_gen.stack_tracker.pop().unwrap());
                    } else {
                        operands.push(stack_item);
                    }
                }

                let result_variable = gen.method_gen.allocate_variable(&operand_type, false, 1);

                writeln!(
                    gen,
                    "{0} = {1} {2} {3} %{5}, %{4}",
                    result_variable[0],
                    instruction.opcode(&operand_type),
                    if **instruction == Instruction::Add() || **instruction == Instruction::Sub() {
                        "nsw"
                    } else {
                        ""
                    },
                    operand_type,
                    operands[0].index,
                    operands[1].index
                );
            }
            Instruction::Ret() => {
                if method.return_type == "void" {
                    writeln!(gen, "ret void");
                } else {
                    if !gen.method_gen.assert_same_type(&method.return_type, 1) {
                        panic!("Method requires at least one operand on stack.")
                    }

                    let operand = gen.method_gen.stack_tracker.last().unwrap();

                    if operand.is_ptr {
                        let variable =
                            gen.method_gen
                                .allocate_variable(&method.return_type, false, 1);

                        writeln!(
                            gen,
                            "{0} = load {1}, {1}* %{2}",
                            variable[0],
                            method.return_type,
                            gen.method_gen.variable_index - 1
                        );
                    }

                    writeln!(
                        gen,
                        "ret {0} %{1}",
                        method.return_type,
                        gen.method_gen
                            .stack_tracker
                            .last()
                            .expect("Expected operand to return but no value on stack")
                            .index
                    );
                }
            }
        }

        last_instruction = instruction.to_owned();
    }

    // method tail
    gen.method_gen.dec_indent();
    gen.writeln("}")
}
