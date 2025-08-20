/*
Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for
Imbriqua Engine project

Imbriqua Structure is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

Imbriqua Structure is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Imbriqua Structure.
If not, see <https://www.gnu.org/licenses/>.
*/

#![warn(dead_code)]
#![warn(missing_docs)]
#![doc = include_str!("lib.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields};

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[proc_macro_derive(XMIIdentity)]
/// Macro derive on implement [XMIIdentity](crate::XMIIdentity) trait to Struct and Enum
pub fn xmi_identity(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
        Data::Enum(DataEnum { .. }) => impl_xmi_identity_for_enum(&ast),
        Data::Struct(DataStruct { .. }) => impl_xmi_identity_for_struct(&ast),
        _ => panic!("XMIIdentity only for Struct and Enum"),
    }
}

// ####################################################################################################

fn impl_xmi_identity_for_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut arms_get_xmi_label = Vec::new();
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
                    arms_get_xmi_label.push(quote! {
                        #name::#variant_ident(c) => c.get_xmi_label(),
                    });
                    arms_get_xmi_id.push(quote! {
                        #name::#variant_ident(c) => c.xmi_id.clone(),
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
            fn get_xmi_label(&self) -> Result<String, anyhow::Error> {
                match self {
                    #(#arms_get_xmi_label)*
                }
            }
            fn get_xmi_id(&self) -> XMIIdLocalReference {
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

// ####################################################################################################

fn impl_xmi_identity_for_struct(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let content = quote! {
        impl XMIIdentification for #name {
            fn get_xmi_label(&self) -> Result<String, anyhow::Error> {
                self.xmi_id.label()
            }
            fn get_xmi_id(&self) -> XMIIdLocalReference {
                self.xmi_id.clone()
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

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[proc_macro_derive(XMIDeserialize)]
pub fn xmi_deserialize(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
        Data::Enum(DataEnum { .. }) => impl_xmi_deserialize_for_enum(&ast),
        Data::Struct(DataStruct { .. }) => impl_xmi_deserialize_for_struct(&ast),
        _ => panic!("XMIDeserialize only for Struct and Enum"),
    }
}

// ####################################################################################################

fn impl_xmi_deserialize_for_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let content = quote! {
        impl XMIDeserialize for #name {}
    };
    content.into()
}

// ####################################################################################################

fn impl_xmi_deserialize_for_struct(ast: &syn::DeriveInput) -> TokenStream {
    // Vérifier que c'est une struct
    let data = match &ast.data {
        Data::Struct(data) => data,
        _ => return TokenStream::from(quote! { #ast }), // Si ce n'est pas une struct, on retourne tel quel
    };

    // Récupérer le nom de la struct
    let name = &ast.ident;
    let mut name2 = ast.ident.clone().to_string();
    name2.push_str("Model");

    // Modifier les champs
    let new_fields = match data.fields.clone() {
        Fields::Named(mut named_fields) => {
            for field in &mut named_fields.named {
                // Exemple : ajouter #[serde(skip_serializing)] à certains champs
                let should_skip = field.ident.as_ref().map_or(false, |ident| ident == "hash");
                let should_skip = true;
                if should_skip {
                    // Vérifier si l'attribut n'existe pas déjà
                    let has_serde_skip = &field.attrs.iter().any(|attr| {
                        attr.path.is_ident("serde")
                            && attr.tokens.to_string().contains("skip_serializing")
                    });
                    if !has_serde_skip {
                        let attr: Attribute = syn::parse_quote! { #[serde(skip_serializing)] };
                        field.attrs.push(attr);
                    }
                }
            }
            // named_fields
            data.fields.clone()
        }
        _ => data.fields.clone(), // Pour simplifier, on ne gère pas les autres cas ici
    };

    // Recréer la struct avec les nouveaux champs
    let content = quote! {
        impl XMIDeserialize for #name {}
        struct #name2 {
            #new_fields
        }
    };
    content.into()
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################
