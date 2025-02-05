#![allow(clippy::assertions_on_result_states, clippy::non_ascii_literal)]

#[macro_use]
mod macros;

use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Stmt;

#[test]
fn test_raw_operator() {
    let stmt = syn::parse_str::<Stmt>("let _ = &raw const x;").unwrap();

    snapshot!(stmt, @r###"
    Local(Local {
        pat: Pat::Wild,
        init: Some(LocalInit {
            expr: Verbatim(`& raw const x`),
        }),
    })
    "###);
}

#[test]
fn test_raw_variable() {
    let stmt = syn::parse_str::<Stmt>("let _ = &raw;").unwrap();

    snapshot!(stmt, @r###"
    Local(Local {
        pat: Pat::Wild,
        init: Some(LocalInit {
            expr: Expr::Reference {
                expr: Expr::Path {
                    path: Path {
                        segments: [
                            PathSegment {
                                ident: "raw",
                                arguments: None,
                            },
                        ],
                    },
                },
            },
        }),
    })
    "###);
}

#[test]
fn test_raw_invalid() {
    assert!(syn::parse_str::<Stmt>("let _ = &raw x;").is_err());
}

#[test]
fn test_none_group() {
    // <Ø async fn f() {} Ø>
    let tokens = TokenStream::from_iter(vec![TokenTree::Group(Group::new(
        Delimiter::None,
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("async", Span::call_site())),
            TokenTree::Ident(Ident::new("fn", Span::call_site())),
            TokenTree::Ident(Ident::new("f", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new())),
        ]),
    ))]);

    snapshot!(tokens as Stmt, @r###"
    Item(Item::Fn {
        vis: Inherited,
        sig: Signature {
            asyncness: Some,
            ident: "f",
            generics: Generics,
            output: Default,
        },
        block: Block,
    })
    "###);
}

#[test]
fn test_let_dot_dot() {
    let tokens = quote! {
        let .. = 10;
    };

    snapshot!(tokens as Stmt, @r###"
    Local(Local {
        pat: Pat::Rest,
        init: Some(LocalInit {
            expr: Expr::Lit {
                lit: 10,
            },
        }),
    })
    "###);
}

#[test]
fn test_let_else() {
    let tokens = quote! {
        let Some(x) = None else { return 0; };
    };

    snapshot!(tokens as Stmt, @r###"
    Local(Local {
        pat: Pat::TupleStruct {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Some",
                        arguments: None,
                    },
                ],
            },
            pat: PatTuple {
                elems: [
                    Pat::Ident {
                        ident: "x",
                    },
                ],
            },
        },
        init: Some(LocalInit {
            expr: Expr::Path {
                path: Path {
                    segments: [
                        PathSegment {
                            ident: "None",
                            arguments: None,
                        },
                    ],
                },
            },
            diverge: Some(Expr::Block {
                block: Block {
                    stmts: [
                        Expr(
                            Expr::Return {
                                expr: Some(Expr::Lit {
                                    lit: 0,
                                }),
                            },
                            Some,
                        ),
                    ],
                },
            }),
        }),
    })
    "###);
}
