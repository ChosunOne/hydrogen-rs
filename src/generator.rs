use crate::parser::NodeExit;

pub(crate) fn generate(root: NodeExit) -> String {
    format!(
        r#"global _start
    _start:
        mov rax, 60
        mov rdi, {}
        syscall"#,
        root.expr.value
    )
}
