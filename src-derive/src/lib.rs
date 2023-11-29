use proc_macro::TokenStream;
use quote::quote;
use deluxe;
use syn;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(conversion))]
struct DistanceSIUnit {
    meter: f64,
    integer: Option<bool>,
}

fn implement_distance_unit(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream>{
    let mut ast: syn::DeriveInput =syn::parse2(input)?;

    let DistanceSIUnit {meter, integer} =deluxe::extract_attributes(&mut ast)?;

    let ident =&ast.ident;

    let eval = match integer {
        None => quote!{ value },
        Some(_) => quote!{value.round() as usize}
    };

    let cast_int_to_f64 = match integer {
        None => quote!{},
        Some(_) => quote!{as f64}
    };


Ok(
    quote!{
            impl Unit for #ident {
                fn new(value: f64) -> Self {
                    Self {value: #eval}
                }
            }

            impl DistanceUnit for #ident {
                const METER: f64 = #meter;
            }

            impl Div<#ident> for f64 {
                type Output = #ident;

                fn div(self, rhs: #ident) -> Self::Output {
                    #ident::new(self / rhs.value #cast_int_to_f64)
                }
            }

            impl Div<f64> for #ident {
                type Output = Self;

                fn div(self, rhs: f64) -> Self::Output {
                    Self::new(self.value #cast_int_to_f64 / rhs)
                }
            }
            impl Mul<f64> for #ident {
                type Output = Self;

                fn mul(self, rhs: f64) -> Self::Output {
                    Self::new(self.value #cast_int_to_f64 * rhs)
                }
            }

            impl Mul<#ident> for f64 {
                type Output = #ident;

                fn mul(self, rhs: #ident) -> Self::Output {
                    #ident::new(self * rhs.value #cast_int_to_f64)
                }
            }
        }
)

}

#[proc_macro_derive(LengthQuantity, attributes(conversion))]
pub fn derive_unit_length(input: TokenStream) -> TokenStream {
    implement_distance_unit(input.into()).unwrap().into()
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(conversion))]
struct TimeSIUnit {
    second: f64,
    integer: Option<bool>,
}

fn implement_time_unit(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream>{
    let mut ast: syn::DeriveInput =syn::parse2(input)?;

    let TimeSIUnit {second, integer} =deluxe::extract_attributes(&mut ast)?;

    let ident =&ast.ident;

    let eval = match integer {
        None => "value",
        Some(_) => "value.round() as usize"
    }.to_string();

    let cast_self_value_to_f64 = match integer {
        None => "self.value",
        Some(_) => "self.value as f64"
    }.to_string();
    let cast_rhs_value_to_f64 = match integer {
        None => "self.value",
        Some(_) => "self.value as f64"
    }.to_string();


    Ok(
        quote!{
            impl Unit for #ident {
                fn new(value: f64) -> Self {
                    Self {value: #eval}
                }
            };

            impl TemporalUnit for #ident { const SECOND: f64 = #second };

            impl Div<#ident> for f64 {
                type Output = #ident;

                fn div(self, rhs: #ident) -> Self::Output {
                    #ident::new(self / #cast_rhs_value_to_f64)
                }
            };

            impl Div<f64> for #ident {
                type Output = Self;

                fn div(self, rhs: f64) -> Self::Output {
                    Self::new(#cast_self_value_to_f64 / rhs)
                }
            };

            impl Mul<f64> for #ident {
                type Output = Self;

                fn mul(self, rhs: f64) -> Self::Output {
                    Self::new(#cast_self_value_to_f64 * rhs)
                }
            };

            impl Mul<#ident> for f64 {
                type Output = #ident;

                fn mul(self, rhs: #ident) -> Self::Output {
                    #ident::new(self * #cast_rhs_value_to_f64)
                }
            };
        }
    )

}

#[proc_macro_derive(TimeQuantity, attributes(conversion))]
pub fn derive_unit_time(input: TokenStream) -> TokenStream {
    implement_time_unit(input.into()).unwrap().into()
}