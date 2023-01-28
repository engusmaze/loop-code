use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Block, LitInt, Result,
};
use syn::{parse2, Ident};

extern crate proc_macro;

struct SimpleCodeLoop {
    loops: usize,
    block: Block,
}
impl Parse for SimpleCodeLoop {
    fn parse(input: ParseStream) -> Result<Self> {
        let loops: LitInt = input.parse()?;
        let block: Block = input.parse()?;

        let loops: usize = loops.base10_parse()?;

        Ok(Self { loops, block })
    }
}

struct AdvancedCodeLoop {
    index_variable: Ident,
    loops: usize,
    number_type: Ident,
    block: Block,
}
impl Parse for AdvancedCodeLoop {
    fn parse(input: ParseStream) -> Result<Self> {
        let index_variable: Ident = input.parse()?;
        let number_type: Ident = input
            .parse()
            .unwrap_or_else(|_| Ident::new("usize", Span::call_site()));
        let loops: LitInt = input.parse()?;
        let block: Block = input.parse()?;

        let loops: usize = loops.base10_parse()?;

        Ok(Self {
            index_variable,
            loops,
            number_type,
            block,
        })
    }
}

#[proc_macro]
pub fn repeat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    if let Ok(code_loop) = parse2::<AdvancedCodeLoop>(input.clone()) {
        let mut code = TokenStream::new();
        for i in 0..code_loop.loops {
            let block = &code_loop.block;
            let variable = &code_loop.index_variable;
            let kind = &code_loop.number_type;
            let lit = LitInt::new(&i.to_string(), Span::call_site());
            quote! {
                {
                    const #variable: #kind = #lit;
                    #block
                }
            }
            .to_tokens(&mut code);
        }

        code.into()
    } else {
        let input: proc_macro::TokenStream = input.into();
        let code_loop = parse_macro_input!(input as SimpleCodeLoop);

        let mut blocks = TokenStream::new();
        for _ in 0..code_loop.loops {
            code_loop.block.to_tokens(&mut blocks);
        }

        blocks.into()
    }
}
