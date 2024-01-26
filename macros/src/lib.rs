#![crate_type = "proc-macro"]

extern crate proc_macro;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use std::iter::Map;
use std::thread::sleep;
use std::time::Duration;
use syn::punctuated::Iter;
use syn::{parse_macro_input, Field, Type};

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            // println!("#####{}", segment.ident.to_string());

            return segment.ident.to_string() == "Option";
        }
    }
    false
}

#[proc_macro_derive(AsjsonRequired)]
pub fn json_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    let expanded = if let syn::Data::Struct(ref data) = input.data {
        // Generate the serialization code for each field that is not Option
        let serialization_code = serialize_fields(data);

        quote! {
            impl #name {
                fn to_json_required(&self) -> serde_json::Value {
                    let mut map = serde_json::Map::new();
                    #( #serialization_code )*
                    serde_json::Value::Object(map)
                }
            }
        }
    } else {
        // Handle other data structures (enums, unions, etc.) if needed
        quote! {
            compile_error!("AsJson can only be derived for structs");
        }
    };

    expanded.into()
}

fn serialize_fields(
    data: &syn::DataStruct,
) -> Map<Iter<Field>, fn(&Field) -> proc_macro2::TokenStream> {
    data.fields.iter().map(|field| proc_macro2::TokenStream::from({
        let field_name = &field.ident;
        let field_type = &field.ty;

        if is_option_type(field_type) {
            quote! {}
        } else {

            quote! {
                    map.insert(stringify!(#field_name).to_string(), serde_json::to_value(&self.#field_name).unwrap_or(serde_json::Value::Null));
                }
        }
    }))
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs"),
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ast = parse_macro_input!(ty as syn::DeriveInput);

        quote! { pub #name: #ty } // set the field to public
    });

    let public_version = quote! {
        pub struct #name { // set the struct to public
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

#[proc_macro_derive(MyTrait)]
pub fn derive_answer_fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Ensure it's deriving for a struct.
    let s = match venial::parse_declaration(proc_macro2::TokenStream::from(item)) {
        Ok(venial::Declaration::Struct(s)) => s,
        Ok(_) => panic!("Can only derive this trait on a struct"),
        Err(_) => panic!("Error parsing into valid Rust"),
    };

    let struct_name = s.name;

    // Get the struct's first field.
    let fields = s.fields;
    let named_fields = match fields {
        venial::StructFields::Named(named_fields) => named_fields,
        _ => panic!("Expected a named field"),
    };

    let inners: Vec<(venial::NamedField, proc_macro2::Punct)> = named_fields.fields.inner;
    if inners.len() != 1 {
        panic!("Expected exactly one named field");
    }

    // Get the name and type of the first field.
    let first_field_name = &inners[0].0.name;
    let first_field_type = &inners[0].0.ty;

    // Extract Id and Record from the type HashMap<Id, Record>
    if first_field_type.tokens.len() != 6 {
        panic!("Expected type T<R, S> for first named field");
    }

    let id = first_field_type.tokens[2].clone();
    let record = first_field_type.tokens[4].clone();

    // Implement MyTrait.
    let generated = quote! {
        impl MyTrait for #struct_name {
            fn foo(&self, id: #id) -> #record { *self.#first_field_name.get(&id).unwrap() }
        }
    };

    proc_macro::TokenStream::from(generated)
}
