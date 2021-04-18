use inflections::Inflect;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::braced;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::token::Colon;
use syn::Ident;
use syn::Type;

struct Field {
    name: Ident,
    kind: Type,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon = input.parse::<Colon>()?;
        let kind = input.parse()?;
        Ok(Self { name, kind })
    }
}

struct AstType {
    name: Ident,
    fields: Vec<Field>,
}

impl Parse for AstType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let contents;
        let _brace = braced!(contents in input);
        let mut fields = vec![];
        while let Ok(field) = contents.parse::<Field>() {
            fields.push(field);
        }
        Ok(Self { name, fields })
    }
}

struct GenerateAstInput {
    base: Ident,
    types: Vec<AstType>,
}

impl Parse for GenerateAstInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base = input.parse()?;
        let mut types = vec![];
        while let Ok(ty) = input.parse::<AstType>() {
            types.push(ty);
        }
        Ok(Self { base, types })
    }
}

#[proc_macro]
pub fn generate_ast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenerateAstInput);
    let base = input.base;
    let visitor_name = format_ident!("{}Visitor", base);
    let visitor = define_visitor(&base, &input.types, &visitor_name);
    let (accept_fn, trait_name) = define_accept_fn(&base, &visitor_name);

    let types = input
        .types
        .iter()
        .map(|ty| {
            let fields = ty
                .fields
                .iter()
                .map(|f| {
                    let name = &f.name;
                    let ty = &f.kind;
                    quote! {#name: #ty}
                })
                .collect::<Vec<_>>();
            let name = format_ident!("{}{}", ty.name, base);
            let ty_name = &ty.name;
            let field_names = ty.fields.iter().map(|f| &f.name).collect::<Vec<_>>();
            let visit_name = format_ident!(
                "visit_{}_{}",
                ty_name.to_string().to_snake_case(),
                base.to_string().to_snake_case()
            );
            quote! {
                pub(crate) struct #name {
                    #(#fields),*
                }
                impl #name {
                    pub(crate) fn new(#(#fields),*) -> Self {
                        Self {
                            #(#field_names),*
                        }
                    }
                }
                impl #trait_name for #name {
                    #accept_fn {
                        visitor.#visit_name(self)
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let base_variants = input.types.iter().map(|ty| {
        let name_upper = format_ident!("{}", ty.name.to_string().to_pascal_case());
        let name = format_ident!("{}{}", ty.name, base);
        quote! {
            #name_upper(#name)
        }
    });

    let match_branches = input
        .types
        .iter()
        .map(|ty| {
            let name = &ty.name;
            quote! {
                Self::#name(x) => x.accept(visitor)
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        pub(crate) enum #base {
            #(#base_variants),*
        }
        trait #trait_name {
            #accept_fn;
        }
        impl #trait_name for #base {
            #accept_fn {
                match self {
                    #(#match_branches),*
                }
            }
        }
        #(#types)*
        #visitor
    };
    output.into()
}

fn define_accept_fn(base: &Ident, visitor_name: &Ident) -> (proc_macro2::TokenStream, Ident) {
    let trait_name = format_ident!("Accept{}Visitor", base);
    (
        quote! {
            fn accept<R>(&self, visitor: &#visitor_name<R>) -> R
        },
        trait_name,
    )
}

fn define_visitor(base: &Ident, types: &[AstType], name: &Ident) -> proc_macro2::TokenStream {
    let visit_fns = types
        .iter()
        .map(|ty| {
            let fn_name = format_ident!(
                "visit_{}_{}",
                ty.name.to_string().to_snake_case(),
                base.to_string().to_snake_case()
            );
            let ty_name = format_ident!("{}{}", ty.name, base);
            let var_name = format_ident!("{}", base.to_string().to_snake_case());
            quote! {
                fn #fn_name(&self, #var_name: &#ty_name) -> R;
            }
        })
        .collect::<Vec<_>>();
    let visitor = quote! {
        pub(crate) trait #name<R> {
            #(#visit_fns)*
        }
    };
    visitor
}
