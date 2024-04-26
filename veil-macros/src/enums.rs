use crate::{
    flags::{ExtractFlags, FieldFlags, FieldFlagsParse},
    fmt::{self, FormatData},
    redact::UnusedDiagnostic,
};
use diff_priv::noise::laplace::laplace_noiser::LaplaceNoiser; // Import LaplaceNoiser
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Default)]
struct EnumVariantFieldFlags {
    variant_flags: Option<FieldFlags>,
    all_fields_flags: Option<FieldFlags>,
}

pub(super) fn derive_redact(
    e: syn::DataEnum,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    name_ident: syn::Ident,
    unused: &mut UnusedDiagnostic,
) -> Result<TokenStream, syn::Error> {
    let top_level_flags = match FieldFlags::extract::<1>("Redact", &attrs, FieldFlagsParse { skip_allowed: false })? {
        [Some(flags)] => {
            if !flags.all || !flags.variant {
                return Err(syn::Error::new(
                    attrs[0].span(),
                    "at least `#[redact(all, variant)]` is required here to redact all variant names",
                ));
            } else if flags.display {
                return Err(syn::Error::new(attrs[0].span(), "`#[redact(display)]` is invalid here"));
            } else {
                Some(flags)
            }
        }
        _ => None,
    };

    let mut variant_flags = Vec::with_capacity(e.variants.len());
    for variant in &e.variants {
        let mut flags = match FieldFlags::extract::<2>(
            "Redact",
            &variant.attrs,
            FieldFlagsParse {
                skip_allowed: top_level_flags.is_some(),
            },
        )? {
            [None, None] => EnumVariantFieldFlags::default(),
            [Some(flags), None] => {
                if flags.all && flags.variant {
                    return Err(syn::Error::new(
                        variant.attrs[0].span(),
                        "`#[redact(all, variant, ...)]` is invalid here, split into two separate attributes instead to apply redacting options to the variant name or all fields respectively",
                    ));
                } else if flags.all {
                    EnumVariantFieldFlags {
                        variant_flags: None,
                        all_fields_flags: Some(flags),
                    }
                } else if flags.variant {
                    if flags.display {
                        return Err(syn::Error::new(
                            variant.attrs[0].span(),
                            "`#[redact(display)]` is invalid here, enum variants are always displayed using std::fmt::Display",
                        ));
                    }
                    EnumVariantFieldFlags {
                        variant_flags: Some(flags),
                        all_fields_flags: None,
                    }
                } else {
                    return Err(syn::Error::new(
                        variant.span(),
                        "please specify at least `#[redact(all, ...)]` or `#[redact(variant, ...)]` first, or both as separate attributes",
                    ));
                }
            }
            [Some(flags0), Some(flags1)] => {
                let mut variant_flags = EnumVariantFieldFlags::default();
                for flags in [flags0, flags1] {
                    if flags.all && flags.variant {
                        return Err(syn::Error::new(
                            variant.span(),
                            "`#[redact(all, variant, ...)]` is invalid here, split into two separate attributes instead to apply redacting options to the variant name or all fields respectively",
                        ));
                    } else if flags.all {
                        if variant_flags.all_fields_flags.is_some() {
                            return Err(syn::Error::new(
                                variant.span(),
                                "a `#[redact(all, ...)]` is already present",
                            ));
                        }
                        variant_flags.all_fields_flags = Some(flags);
                    } else if flags.variant {
                        if variant_flags.variant_flags.is_some() {
                            return Err(syn::Error::new(
                                variant.span(),
                                "a `#[redact(variant, ...)]` is already present",
                            ));
                        }
                        variant_flags.variant_flags = Some(flags);
                    } else {
                        return Err(syn::Error::new(
                            variant.span(),
                            "please specify at least `#[redact(all, ...)]` or `#[redact(variant, ...)]` first, or both as separate attributes",
                        ));
                    }
                }
                variant_flags
            }
            [None, ..] => unreachable!(),
        };

        if flags.variant_flags.is_none() {
            if let Some(top_level_flags) = top_level_flags {
                flags.variant_flags = Some(top_level_flags);
            }
        }

        variant_flags.push(flags);
    }

    let variant_idents = e.variants.iter().map(|variant| &variant.ident);

    let variant_destructures = e.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            let idents = named.iter().map(|field| field.ident.as_ref().unwrap());
            quote! { { #(#idents),* } }
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let args = (0..unnamed.len()).map(|i| syn::Ident::new(&format!("arg{}", i), unnamed.span()));
            quote! { ( #(#args),* ) }
        }
        syn::Fields::Unit => Default::default(),
    });

    // Create a LaplaceNoiser instance
    let noiser = LaplaceNoiser::new(0.1, 3, 0.1); // could adjust the noise parameters

    let mut variant_bodies = Vec::with_capacity(e.variants.len());
    for (variant, flags) in e.variants.iter().zip(variant_flags.into_iter()) {
        let variant_name = variant.ident.to_string();
        let variant_name = if let Some(flags @ FieldFlags { skip: false, .. }) = &flags.variant_flags {
            let flags = FieldFlags {
                display: true,
                ..*flags
            };
            let redact = fmt::generate_redact_call(quote! { &#variant_name }, false, &flags, unused);
            quote! { format!("{:?}", #redact).as_str() }
        } else {
            variant_name.into_token_stream()
        };

        let noised_fields = match &variant.fields {
            syn::Fields::Named(named) => {
                let noised_fields = named.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    if field_name == "number" { //  'number' is a sensitive field
                        quote! {
                            let noised_value = noiser.add_noise(self.#field_name);
                            fmt.write_str(&format!("{:?}", noised_value))?;
                        }
                    } else {
                        quote! {
                            fmt.write_str(&format!("{:?}", self.#field_name))?;
                        }
                    }
                });

                quote! { #(#noised_fields);*; }
            }
            _ => Default::default(),
        };

        variant_bodies.push(match &variant.fields {
            syn::Fields::Named(named) => {
                FormatData::FieldsNamed(named).impl_debug(variant_name, flags.all_fields_flags, false, unused)?
            }
            syn::Fields::Unnamed(unnamed) => {
                FormatData::FieldsUnnamed(unnamed).impl_debug(variant_name, flags.all_fields_flags, false, unused)?
            }
            syn::Fields::Unit => {
                if flags.all_fields_flags.is_some() {
                    return Err(syn::Error::new(
                        variant.attrs[0].span(),
                        "unit structs do not need redacting as they contain no data",
                    ));
                } else {
                    quote! { fmt.write_str(#variant_name)? }
                }
            }
        });

        variant_bodies.push(noised_fields);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::std::fmt::Debug for #name_ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #[allow(unused)]
                let alternate = fmt.alternate();

                match self {
                    #(Self::#variant_idents #variant_destructures => { #variant_bodies; },)*
                }

                Ok(())
            }
        }
    }
    .into())
}
