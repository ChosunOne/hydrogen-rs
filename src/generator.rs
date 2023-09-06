use crate::parser::{NodeBinExpr, NodeExpr, NodeProgram, NodeStatement, NodeTerm};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Identifier {0} already used")]
    IdentifierInUse(String),
    #[error("'{0}' is not defined")]
    IdentifierNotFound(String),
}

impl NodeProgram {
    pub fn generate(&self) -> Result<String, GeneratorError> {
        let mut asm = "global _start\n_start:\n".to_owned();
        let mut generator = Generator::new();
        for statement in &self.statements {
            asm.push_str(&statement.generate(&mut generator)?);
        }

        Ok(asm)
    }
}

struct Variable {
    stack_location: usize,
}

pub(crate) struct Generator {
    stack_location: usize,
    variables: HashMap<String, Variable>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            stack_location: 0,
            variables: HashMap::new(),
        }
    }

    pub fn push(&mut self, register: &str) -> String {
        self.stack_location += 1;
        format!("\tpush {register}\n")
    }

    pub fn pop(&mut self, register: &str) -> String {
        self.stack_location -= 1;
        format!("\tpop {register}\n")
    }
}

pub(crate) trait Generate {
    fn generate(&self, generator: &mut Generator) -> Result<String, GeneratorError>;
}

impl Generate for NodeStatement {
    #[must_use]
    fn generate(&self, generator: &mut Generator) -> Result<String, GeneratorError> {
        match self {
            Self::End => Ok("\tmov rax, 60\n\tmov rdi, 0\n\tsyscall\n".into()),
            Self::Exit(expr) => {
                let mut asm = expr.generate(generator)?;
                asm.push_str("\tmov rax, 60\n");
                asm.push_str(generator.pop("rdi").as_str());
                asm.push_str("\tsyscall\n");
                Ok(asm)
            }
            Self::Let((ident, expr)) => {
                if generator.variables.contains_key(&ident.to_string()) {
                    return Err(GeneratorError::IdentifierInUse(ident.to_string()));
                }
                let variable = Variable {
                    stack_location: generator.stack_location,
                };
                generator.variables.insert(ident.to_string(), variable);
                let asm = expr.generate(generator)?;
                Ok(asm)
            }
        }
    }
}

impl Generate for NodeExpr {
    fn generate(&self, generator: &mut Generator) -> Result<String, GeneratorError> {
        match self {
            Self::Term(t) => t.generate(generator),
            Self::BinExpr(b) => b.generate(generator),
        }
    }
}

impl Generate for NodeBinExpr {
    fn generate(&self, generator: &mut Generator) -> Result<String, GeneratorError> {
        match self {
            Self::Add(bin_expr) => {
                let mut asm = bin_expr.lhs.generate(generator)?;
                asm.push_str(&bin_expr.rhs.generate(generator)?);
                asm.push_str(&generator.pop("rax"));
                asm.push_str(&generator.pop("rbx"));
                asm.push_str("\tadd rax, rbx\n");
                asm.push_str(&generator.push("rax"));
                Ok(asm)
            }
            _ => todo!(),
        }
    }
}

impl Generate for NodeTerm {
    fn generate(&self, generator: &mut Generator) -> Result<String, GeneratorError> {
        match self {
            Self::IntLit(t) => {
                let mut asm = format!("\tmov rax, {}\n", t.to_string());
                asm.push_str(generator.push("rax").as_str());
                Ok(asm)
            }
            Self::Ident(t) => {
                let variable = generator
                    .variables
                    .get(&t.to_string())
                    .ok_or(GeneratorError::IdentifierNotFound(t.to_string()))?;

                let asm = generator.push(&format!(
                    "QWORD [rsp + {}]",
                    (generator.stack_location - variable.stack_location - 1) * 8
                ));
                Ok(asm)
            }
        }
    }
}
