extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(XMIIdentification)]
pub fn xmi_tools(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
        Data::Enum(DataEnum { .. }) => impl_xmi_tools_for_enum(&ast),
        Data::Struct(DataStruct { .. }) => impl_xmi_tools_for_struct(&ast),
        _ => panic!("XMIIdentification only for Struct and Enum"),
    }
}

fn impl_xmi_tools_for_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut arms_get_xmi_id_field = Vec::new();
    let mut arms_get_xmi_id_object = Vec::new();

    if let Data::Enum(DataEnum { variants, .. }) = &ast.data {
        for (_, variant) in variants.iter().enumerate() {
            let variant_ident = &variant.ident;
            match &variant.fields {
                Fields::Unit => {
                    panic!("Fields::Unit");
                }
                Fields::Unnamed(fields) => {
                    if fields.unnamed.iter().len() != 1 {
                        panic!("Fields::Unnamed");
                    }
                    arms_get_xmi_id_field.push(quote! {
                        #name::#variant_ident(c) => c.get_xmi_id_field(),
                    });
                    arms_get_xmi_id_object.push(quote! {
                        #name::#variant_ident(c) => c.get_xmi_id_object(),
                    });
                }
                Fields::Named(_) => {
                    panic!("Fields::Named");
                }
            }
        }
    }

    // 'ast' must refer to an Enum
    let name = &ast.ident;
    let content = quote! {
        impl XMIIdentification for #name {
            fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
                match self {
                    #(#arms_get_xmi_id_field)*
                }
            }
            fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
                match self {
                    #(#arms_get_xmi_id_object)*
                }
            }
        }
    };
    content.into()
}

fn impl_xmi_tools_for_struct(ast: &syn::DeriveInput) -> TokenStream {
    fn impl_xmi_tools_for_struct_get_functions(_ast: &syn::DeriveInput) -> TokenStream2 {
        quote!(
            fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
                self.xmi_id.label()
            }
            fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
                Ok(self.xmi_id.get_object_id())
            }
        )
    }

    // 'ast' must refer to an Struct
    let name = &ast.ident;
    let quote_get_functions = impl_xmi_tools_for_struct_get_functions(&ast);
    let content = quote! {
        impl XMIIdentification for #name {
            #quote_get_functions
        }
    };
    content.into()
}
