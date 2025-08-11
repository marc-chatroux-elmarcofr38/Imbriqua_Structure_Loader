extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(XMIIdentity)]
pub fn xmi_tools(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
        Data::Enum(DataEnum { .. }) => impl_xmi_tools_for_enum(&ast),
        Data::Struct(DataStruct { .. }) => impl_xmi_tools_for_struct(&ast),
        _ => panic!("XMIIdentity only for Struct and Enum"),
    }
}

fn impl_xmi_tools_for_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut arms_get_xmi_id_field = Vec::new();
    let mut arms_get_xmi_id = Vec::new();

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
                    arms_get_xmi_id.push(quote! {
                        #name::#variant_ident(c) => &c.xmi_id,
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
            fn get_xmi_id(&self) -> &XMIIdLocalReference {
                match self {
                    #(#arms_get_xmi_id)*
                }
            }
        }
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.get_xmi_id() == other.get_xmi_id()
            }
        }
        impl Eq for #name {}
        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.get_xmi_id().cmp(&other.get_xmi_id())
            }
        }
        impl XMIIdentity for #name {}
    };
    content.into()
}

fn impl_xmi_tools_for_struct(ast: &syn::DeriveInput) -> TokenStream {
    // 'ast' must refer to an Struct
    let name = &ast.ident;
    let content = quote! {
        impl XMIIdentification for #name {
            fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
                self.xmi_id.label()
            }
            fn get_xmi_id(&self) -> &XMIIdLocalReference {
                &self.xmi_id
            }
        }
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.xmi_id == other.xmi_id
            }
        }
        impl Eq for #name {}
        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.xmi_id.cmp(&other.xmi_id)
            }
        }
        impl XMIIdentity for #name {}
    };
    content.into()
}
