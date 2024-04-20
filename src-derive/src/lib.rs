use deluxe;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(conversion))]
struct DistanceSIUnit {
    meter: f32,
    integer: Option<bool>,
}

fn implement_distance_unit(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: syn::DeriveInput = syn::parse2(input)?;

    let DistanceSIUnit { meter, integer } = deluxe::extract_attributes(&mut ast)?;

    let ident = &ast.ident;

    let discreetize = match integer {
        None | Some(false) => quote! { value },
        Some(true) => quote! {value.round() as usize},
    };

    let cast_int_to_f32 = match integer {
        None | Some(false) => quote! {},
        Some(true) => quote! {as f32},
    };

    Ok(quote! {
        impl Unit for #ident {
            fn new(value: f32) -> Self {
                Self {value: #discreetize}
            }
        }

        impl DistanceUnit for #ident {
            const METER: f32 = #meter;
        }

        impl Div<#ident> for f32 {
            type Output = f32;

            fn div(self, rhs: #ident) -> Self::Output {
                self / rhs.value #cast_int_to_f32
            }
        }

        impl Div<f32> for #ident {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self::new(self.value #cast_int_to_f32 / rhs)
            }
        }
        impl Mul<f32> for #ident {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::new(self.value #cast_int_to_f32 * rhs)
            }
        }

        impl Mul<#ident> for f32 {
            type Output = f32;

            fn mul(self, rhs: #ident) -> Self::Output {
                self * rhs.value #cast_int_to_f32
            }
        }
    })
}

#[proc_macro_derive(LengthQuantity, attributes(conversion))]
pub fn derive_unit_length(input: TokenStream) -> TokenStream {
    implement_distance_unit(input.into()).unwrap().into()
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(conversion))]
struct TimeSIUnit {
    second: f32,
    integer: Option<bool>,
}

fn implement_time_unit(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: syn::DeriveInput = syn::parse2(input)?;

    let TimeSIUnit { second, integer } = deluxe::extract_attributes(&mut ast)?;

    let ident = &ast.ident;

    let discreetize = match integer {
        None | Some(false) => quote! { value },
        Some(_) => quote! {value.round() as usize},
    };

    let cast_int_to_f32 = match integer {
        None | Some(false) => quote! {},
        Some(true) => quote! {as f32},
    };

    Ok(quote! {
        impl Unit for #ident {
            fn new(value: f32) -> Self {
                Self {value: #discreetize}
            }
        }

        impl TemporalUnit for #ident {
            const SECOND: f32 = #second;
        }

        impl Div<#ident> for f32 {
            type Output = f32;

            fn div(self, rhs: #ident) -> Self::Output {
                self / rhs.value #cast_int_to_f32
            }
        }

        impl Div<f32> for #ident {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self::new(self.value #cast_int_to_f32 / rhs)
            }
        }
        impl Mul<f32> for #ident {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::new(self.value #cast_int_to_f32 * rhs)
            }
        }

        impl Mul<#ident> for f32 {
            type Output = f32;

            fn mul(self, rhs: #ident) -> Self::Output {
                self * rhs.value #cast_int_to_f32
            }
        }
    })
}

#[proc_macro_derive(TimeQuantity, attributes(conversion))]
pub fn derive_unit_time(input: TokenStream) -> TokenStream {
    implement_time_unit(input.into()).unwrap().into()
}
