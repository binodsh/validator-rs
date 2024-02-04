use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Lit, LitInt, LitStr};

#[proc_macro_derive(Validate, attributes(validate, field_validate))]
#[proc_macro_error]
pub fn validate_derive_macro(input: TokenStream) -> TokenStream {
    // parse
    let ast = syn::parse(input).unwrap();
    //  generate
    impl_validation(&ast)
}


fn pretty_print_quote(ts: &proc_macro2::TokenStream) {
    match syn::parse_file(ts.to_string().as_str()) {
        Ok(val) => {
            println!("{}", prettyplease::unparse(&val))
        }
        Err(e) => {
            println!("error while printing: {:#?}", e)
        }
    };
}

fn impl_validation(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    let fields = collect_fields(&ast);
    let validations = build_validations(&fields);

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let validator_func_quote = quote::quote!(
        impl #impl_generics ::validator_rs::Validate for #ident #ty_generics #where_clause {
            fn validate(&self) -> ::std::result::Result<(), Vec<::validator_rs::ValidationError>> {
                use ::validator_rs::ValidationError;
                use ::validator_rs::validation::min_length::validate_min_length;
                use ::validator_rs::validation::max_length::validate_max_length;
                use ::validator_rs::validation::email::validate_email;
                use ::validator_rs::validation::contains::validate_contains;

                let mut errors: Vec<ValidationError> = vec![];

                #(#validations)*

                if errors.len() > 0 {
                    Err(errors)
                } else {
                    Ok(())
                }
            }
        }
    );

    pretty_print_quote(&validator_func_quote);

    validator_func_quote.into()
}

fn collect_field_quotes(field: &syn::Field) -> Vec<proc_macro2::TokenStream> {
    let mut quotes: Vec<proc_macro2::TokenStream> = vec![];
    let ident = field.ident.clone().unwrap();
    let field_name = field.ident.clone().unwrap().to_string();
    

    for attr in field.attrs.iter() {
        if attr.path().is_ident("validate") {
            let _ = attr.parse_nested_meta(|meta| {
                // TODO: validate attribute value type and input value type in each attribute
                // for e.g. min_length should only support positive integer type in attribute value
                // also min_length should be used with String and list types 
                

                // TODO: add support for following cases
                // ip
                // must match
                // regex
                // url
                if meta.path.is_ident("min_length") {
                    let value = extract_lit_int(&meta)?;

                    let ts = quote!(
                        if !validate_min_length(&self.#ident, #value) {
                            errors.push(ValidationError::new(#field_name.to_string(), "value is not min".to_string()));
                        }
                    );
                    quotes.push(ts);

                } else if meta.path.is_ident("max_length") {
                    let value = extract_lit_int(&meta)?;

                    let ts = quote!(
                        if !validate_max_length(&self.#ident, #value) {
                            errors.push(ValidationError::new(#field_name.to_string(), "value is not max".to_string()));
                        }
                    );
                    quotes.push(ts);

                } else if meta.path.is_ident("email") {
                    let ts = quote!(
                        if !validate_email(self.#ident.as_str()) {
                            errors.push(ValidationError::new(#field_name.to_string(), "value is not email".to_string()));
                        }
                    );

                    quotes.push(ts);
                } else if meta.path.is_ident("contains") {
                    let value = extract_lit_str(&meta)?;
                    let ts = quote!(
                        if !validate_contains(self.#ident.as_str(), #value) {
                            errors.push(ValidationError::new(#field_name.to_string(), "value does not contain".to_string()));
                        }
                    );

                    quotes.push(ts);
                } else if meta.path.is_ident("does_not_contain") {
                    let value = extract_lit_str(&meta)?;
                    let ts = quote!(
                        if validate_contains(self.#ident.as_str(), #value) {
                            errors.push(ValidationError::new(#field_name.to_string(), "value contains".to_string()));
                        }
                    );

                    quotes.push(ts);
                } else {
                    abort!(meta.path.span(), "Unsuported attribute. Supported attributes are contains, does_not_contain, min_length, max_length & email")
                }

                Ok(())
            });
        }
    }

    quotes
}

fn extract_lit_int(meta: &syn::meta::ParseNestedMeta<'_>) -> Result<LitInt, syn::Error> {
    let expr: Lit = match meta.value()?.parse() {
        Ok(val) => {val},
        Err(_) => {
                abort!(meta.path.span(), "should provide some integer value")
        },
    };
    let s = match expr {
        Lit::Int(ref int_value) => {{
            println!("integer value: {:?}", expr);
            int_value
        }},

        _ => {
            abort!(expr.span(), "only integer type supported")
        }
    };
    Ok(s.clone())
}

fn extract_lit_str(meta: &syn::meta::ParseNestedMeta<'_>) -> Result<LitStr, syn::Error> {
    let expr: Lit = match meta.value()?.parse() {
        Ok(val) => {val},
        Err(_) => {
                abort!(meta.path.span(), "should provide some string value")
        },
    };
    let s = match expr {
        Lit::Str(ref str_value) => {{
            str_value
        }},

        _ => {
            abort!(expr.span(), "only integer type supported")
        }
    };
    Ok(s.clone())
}



fn build_validations(fields: &Vec<syn::Field>) -> Vec<proc_macro2::TokenStream> {
    let mut result = vec![];
    for field in fields {
        let mut field_quotes = collect_field_quotes(&field);
        result.append(&mut field_quotes);
    }

    result
}

fn collect_fields(ast: &syn::DeriveInput) -> Vec<syn::Field> {
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                abort!(
                    fields.span(),
                    "struct has unnamed fields";
                    help = "#[derive(Validate)] can only be used on structs with named fields";
                );
            }
            fields.iter().cloned().collect::<Vec<_>>()
        }
        _ => abort!(
            ast.span(),
            "#[derive(Validate)] can only be used with structs"
        ),
    }
}
