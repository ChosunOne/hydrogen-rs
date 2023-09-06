$$
\begin{align}
[\text{Prog}] &\to [\text{Stmt}]^* \\
[\text{Stmt}] &\to 
\begin{cases} 
    \text{Exit}([\text{Expr}]); \\
    \text{Let}\space \text{Ident} = [\text{Expr}]; \\
\end{cases} \\
[\text{Expr}] &\to 
\begin{cases}
    \text{[Term]} \\ 
    \text{[BinExpr]}
\end{cases} \\
[\text{BinExpr}] &\to 
[\text{Expr}] \oplus [\text{Expr}] \quad \text{prec} = i \\
[\text{Term}] &\to
\begin{cases}
    \text{Int\_Lit} \\
    \text{Ident}
\end{cases}

\end{align}
$$

