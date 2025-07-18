//! Procedural Macros for the [`CookbookRS`] crate
//!
//! These derive macros rely on several common attributes which are shown below, to correctly
//! generate the desired layout of code.
//!
//! The derive implementation of [`WidgetRef`] and [`StatefulWidgetRef`] should produce the same
//! visual output, with the exception of styling or other changes produced with states.
//!
//! The layout renders each field as a widget not marked with the `#[cookbook(skip)]` attribute or
//! either `#[cookbook(left_field)]` or `#[cookbook(right_field)]` as the last two are special
//! cased. The field widgets are then laid out in a vertical [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html).
//!
//! # Struct Attributes
//! - `state_struct` is the name of the struct that holds the state information for the struct that
//!   [`StatefulWidgetRef`] is being derived on. this is case sensitive. It is only processed if
//!   deriving [`StatefulWidgetRef`] and ignored otherwise.
//!
//! # Field Attributes
//! - `display_order` is an integer that determines the order the field will be displayed. It is
//!   used as follows: `display_order = 2`.
//! - `constraint_type` is matched against the values of
//!   [`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html) and
//!   determines the type of constraint for each field. It supports all values except `Ratio`. It is
//!   used as follows: `constraint_type = min`. The first character is not case sensitive.
//! - `constraint_value` is an integer that is used as the value inside the `Constraint`. It is
//!   used as follows: `constraint_value = 5`
//! - `display_widget` is used to select the type of widget to use to display the value of the
//!   field. If not specified, will default to `Paragraph`.
//! - `display_widget_state`
//! - `left_field` is used to select the field that will be displayed as a count in the left hand
//!   info box.
//! - `right_field` is used to select the field that will be displayed as a count in the right hand
//!   info box.
//! - `skip` will skip the field from being rendered.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{Data, DataStruct, DeriveInput, Expr, Fields, Ident, Lit, Meta, Token, Type, parse_macro_input, spanned::Spanned};

use std::collections::BTreeMap;
use std::num::Saturating;

///[`stateful_widget_ref_derive`] is the outer derive function for the [`StatefulWidgetRef`]
///trait on structs with named fields.
#[proc_macro_derive(StatefulWidgetRef, attributes(cookbook))]
pub fn stateful_widget_ref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    widget_ref_expand(input, true)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
///[`widget_ref_derive`] is the outer derive function for the [`WidgetRef`]
///trait on structs with named fields
#[proc_macro_derive(WidgetRef, attributes(cookbook))]
pub fn widget_ref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    widget_ref_expand(input, false)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Implementation of [`StatefulWidgetRef`] and [`WidgetRef`] derive
#[allow(clippy::too_many_lines)]
fn widget_ref_expand(input: DeriveInput, stateful: bool) -> syn::Result<TokenStream2> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "This derive macro only works on structs with named fields.",
            ));
        }
    };

    let struct_name = &input.ident;

    let mut constraints_code = BTreeMap::new();
    let mut field_display_code = BTreeMap::new();
    // field_enum_mapping_code contains the code that creates an enumeration of all fields in
    // struct for use by application code
    let mut field_enum_mapping_code = BTreeMap::new();
    // field_offset_enum_code contains the code that creates an enumeration of field rendering
    // offsets for text editing
    let mut field_offset_enum_code = BTreeMap::new();
    let mut len_check_fn_code = TokenStream2::new();
    let field_enum_name = format_ident!("{}Fields", struct_name);
    let field_offset_enum_name = format_ident!("{}FieldOffset", struct_name);
    let mut field_offset_value: u16 = 0;

    let mut total_field_height: Saturating<u16> = Saturating(0);
    let mut left_field = None;
    let mut right_field = None;
    let mut left_lower_field_title: Option<String> = None;
    let mut right_lower_field_title: Option<String> = None;

    let mut state_struct = String::new();
    //This is checking outer attributes on struct, not on fields
    if stateful {
        state_struct = {
            let mut state_struct_value = None;
            for attr in &input.attrs {
                match &attr.meta {
                    // Outer attribute will always be of form Meta::List as we are looking for
                    // cookbook(__)
                    // this path is the cookbook in cookbook("display_order")
                    Meta::List(primary_meta) if primary_meta.path.is_ident("cookbook") => {
                        primary_meta.parse_nested_meta(|secondary_meta| {
                            if secondary_meta.path.is_ident("state_struct") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error(
                                                "The `cookbook(state_struct)` attribute must be set equal to a literal value",
                                            ));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta
                                                .error("The `cookbook(state_struct)` attribute must be set equal to a string"));
                                        };
                                        state_struct_value = Some(lit_str.value());
                                        Ok(())
                                    }
                                    Err(_) => Err(secondary_meta.error(
                                        "The `cookbook(state_struct) attribute must be called as a NameValue attribute type",
                                    )),
                                }
                            } else {
                                Ok(())
                            }
                        })?;
                    }
                    _ => {}
                }
            }
            state_struct_value.ok_or(syn::Error::new_spanned(
                &input,
                "No `cookbook(state_struct)` specified during `StatefulWidgetRef` derive.",
            ))
        }?;
    }
    //TODO: need to fix styling here
    for f in fields {
        let mut skip = false;
        //indicates if field is used to fill in a value in one of the two bottom fields
        let mut bottom_field = false;

        if let Some(field_name) = f.ident.clone() {
            let block_name = format_ident!("{}_block", field_name);
            let paragraph_name = format_ident!("{}_paragraph", field_name);
            let field_text_style_name = format_ident!("{}_text_style", field_name);
            let field_block_style_name = format_ident!("{}_block_style", field_name);
            let field_block_border_style_name = format_ident!("{}_block_border_style", field_name);
            #[expect(clippy::single_char_pattern)]
            let field_title = to_ascii_titlecase(field_name.to_string().replace("_", " ").as_str());
            let field_enum_variant = format_ident!("{}", to_camelcase_from_snake_case(field_name.to_string().as_str()));
            let mut display_order: Option<usize> = None;
            let mut constraint_type: Option<String> = None;
            let mut constraint_value: Option<u16> = None;
            //this is the default widget for displaying text
            let default_widget_type = format_ident!("Paragraph");
            let mut widget_type = default_widget_type.clone();
            let mut widget_state: Option<Ident> = None;
            let mut widget_options = Vec::new();

            // handle remaining attributes
            for attr in &f.attrs {
                match &attr.meta {
                    // Want outer attribute to always be a Meta::List
                    // this path is the cookbook in cookbook("display_order")
                    Meta::List(primary_meta) if primary_meta.path.is_ident("cookbook") => {
                        // now parse the stuff inside the parenthesis
                        primary_meta.parse_nested_meta(|secondary_meta| {
                            // #[cookbook(skip)]
                            if secondary_meta.path.is_ident("skip") {
                                skip = true;
                            }

                            if secondary_meta.path.is_ident("display_order") {
                                // value() advances meta.input past the = in the input. Will error
                                // if the = is not present.
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(display_order)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Int(ref lit_int) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(display_order)` attribute must be set equal to an integer"));
                                        };

                                        display_order = Some(lit_int.base10_parse::<usize>()?);
                                        Ok(())
                                    }
                                    Err(_) => Err(secondary_meta.error("The `cookbook(display_order)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if secondary_meta.path.is_ident("constraint_type") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(constraint_type)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(constraint_type)` attribute must be set equal to an string"));
                                        };
                                        match lit_str.value().as_str() {
                                            "Min" | "min" => {
                                                constraint_type = Some("Min".to_string());
                                                Ok(())
                                            }
                                            "Max" | "max" => {
                                                constraint_type = Some("Max".to_string());
                                                Ok(())
                                            }
                                            "Length" | "length" => {
                                                constraint_type = Some("Length".to_string());
                                                Ok(())
                                            }
                                            "Percentage" | "percentage" => {
                                                constraint_type = Some("Percentage".to_string());
                                                Ok(())
                                            }
                                            "Fill" | "fill" => {
                                                constraint_type = Some("Fill".to_string());
                                                Ok(())
                                            }
                                            "Ratio" | "ratio" => {
                                                return Err(secondary_meta.error("Ratio constraint type in attribute `cookbook(constraint_type)` is not supported by this derive macro"));
                                            }
                                            x => {
                                                let err_string = format!("Constraint type `cookbook(constraint = {x})` is not recognized");
                                                return Err(secondary_meta.error(err_string));
                                            }
                                        }
                                    }
                                    Err(_) => Err(secondary_meta.error("The `cookbook(constraint_type)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if secondary_meta.path.is_ident("constraint_value") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(constraint_value)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Int(ref lit_int) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(constraint_value)` attribute must be set equal to an integer"));
                                        };
                                        constraint_value = Some(lit_int.base10_parse::<u16>()?);
                                        Ok(())
                                    }

                                    Err(_) => Err(secondary_meta.error("The `cookbook(constraint_value)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if secondary_meta.path.is_ident("display_widget") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(display_widget)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(display_widget)` attribute must be set equal to an string"));
                                        };
                                        // set to Paragraph by default
                                        //TODO: perform validation here
                                        widget_type = format_ident!("{}", lit_str.value());
                                        Ok(())
                                    }

                                    Err(_) => Err(secondary_meta.error("The `cookbook(display_widget)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if secondary_meta.path.is_ident("display_widget_state") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(display_widget_state)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(display_widget_state)` attribute must be set equal to an string"));
                                        };
                                        widget_state = Some(format_ident!("{}", lit_str.value()));
                                        Ok(())
                                    }

                                    Err(_) => Err(secondary_meta.error("The `cookbook(display_widget_state)` attribute must be called as a NameValue attribute type")),
                                }

                                // called like #[cookbook(display_widget_options(A, B, C, D))]
                                // where A, B, C, D will become the options in the widget
                            } else if secondary_meta.path.is_ident("display_widget_options") {
                                secondary_meta.parse_nested_meta(|tertiary_meta| {
                                    widget_options.push(tertiary_meta.path.clone());
                                    Ok(())
                                })
                            } else if secondary_meta.path.is_ident("left_field") {
                                // checking to make sure this attr is a path and doesn't have any values
                                // associated with it
                                // checking to make sure parsing the value errors.
                                // TODO: see if this same approach works for Meta::List
                                //
                                // check if left_field is already set. Have to check here, rather
                                // at the beginning as it interferes with other attribute checks
                                if left_field.is_some() {
                                    return Err(syn::Error::new_spanned(f, "The `cookbook(left_field)` attribute was specified more than once. It must only be specified on one field"));
                                }
                                if secondary_meta.value().is_err() {
                                    left_field = Some(field_name.clone());
                                    left_lower_field_title = Some("no_field_title_specified".to_string());
                                    bottom_field = true;
                                    Ok(())
                                } else {
                                    return Err(secondary_meta.error("The `cookbook(left_field)` attribute must not be called with a value"));
                                }
                                // this is comparing the actual enum variant, and not the
                                // values within
                            } else if secondary_meta.path.is_ident("right_field") {
                                // checking to make sure this attr is a path and doesn't have any values
                                // associated with it
                                // checking to make sure parsing the value errors.
                                // TODO: see if this same approach works for Meta::List
                                //
                                // check if left_field is already set. Have to check here, rather
                                // at the beginning as it interferes with other attribute checks
                                if right_field.is_some() {
                                    return Err(syn::Error::new_spanned(f, "The `cookbook(right_field)` attribute was specified more than once. It must only be specified on one field"));
                                }
                                if secondary_meta.value().is_err() {
                                    right_field = Some(field_name.clone());
                                    bottom_field = true;
                                    Ok(())
                                } else {
                                    return Err(secondary_meta.error("The `cookbook(right_field)` attribute must not be called with a value"));
                                }
                            } else if secondary_meta.path.is_ident("left_field_title") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(left_field_title)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(left_field_title)` attribute must be set equal to an string"));
                                        };
                                        left_lower_field_title = Some(lit_str.value());
                                        Ok(())
                                    }

                                    Err(_) => Err(secondary_meta.error("The `cookbook(left_field_title)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if secondary_meta.path.is_ident("right_field_title") {
                                match secondary_meta.value() {
                                    Ok(value) => {
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(secondary_meta.error("The `cookbook(right_field_title)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(secondary_meta.error("The `cookbook(right_field_title)` attribute must be set equal to an string"));
                                        };
                                        right_lower_field_title = Some(lit_str.value());
                                        Ok(())
                                    }

                                    Err(_) => Err(secondary_meta.error("The `cookbook(right_field_title)` attribute must be called as a NameValue attribute type")),
                                }
                            } else {
                                //continue;
                                Ok(())
                            }
                        })?;
                    }
                    _ => {
                        // ignore any field attributes that are not syn::Meta::List() types with path
                    }
                }
            }

            if bottom_field {
                //https://users.rust-lang.org/t/derive-macro-determine-if-field-implements-trait/109417/6
                let field_type = &f.ty;
                let field_span = f.span();
                len_check_fn_code = quote_spanned! {field_span=>
                    #[expect(clippy::ptr_arg)] //TODO fix this
                    fn _must_have_len_method_returning_usize(x: &#field_type)-> usize {x.len()}
                };
            }
            if skip && !bottom_field {
                continue;
            }
            //only require these fields on fields that are not skip and not bottom fields
            if display_order.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(
                    f,
                    "`the `cookbook(display_order)` attribute is not specified",
                ));
            }
            if constraint_type.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(
                    f,
                    "`the `cookbook(constraint_type)` attribute is not specified",
                ));
            }

            if constraint_value.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(
                    f,
                    "`the `cookbook(constraint_value)` attribute is not specified",
                ));
            }
            if !bottom_field {
                // unwrap_or_default() here is ok as these are all checked for None above here
                total_field_height += constraint_value.unwrap_or_default();
                let mut funct_args = syn::punctuated::Punctuated::new();
                let constraint_value_inner = constraint_value.unwrap_or_default();
                funct_args.push(syn::Type::Verbatim(quote!(#constraint_value_inner)));

                let mut constraint_path = syn::punctuated::Punctuated::new();
                constraint_path.push(syn::PathSegment {
                    ident: format_ident!("ratatui"),
                    arguments: syn::PathArguments::None,
                });
                constraint_path.push_punct(Token![::](proc_macro2::Span::mixed_site()));
                constraint_path.push(syn::PathSegment {
                    ident: format_ident!("layout"),
                    arguments: syn::PathArguments::None,
                });
                constraint_path.push_punct(Token![::](proc_macro2::Span::mixed_site()));
                constraint_path.push(syn::PathSegment {
                    ident: format_ident!("Constraint"),
                    arguments: syn::PathArguments::None,
                });
                constraint_path.push_punct(Token![::](proc_macro2::Span::mixed_site()));
                constraint_path.push(syn::PathSegment {
                    // this was where the empty ident was coming from, on bottom_field fields
                    ident: format_ident!("{}", constraint_type.unwrap_or_default()),

                    arguments: syn::PathArguments::Parenthesized(syn::ParenthesizedGenericArguments {
                        paren_token: syn::token::Paren::default(),
                        inputs: funct_args,
                        output: syn::ReturnType::Default,
                    }),
                });
                let constraint = syn::Path {
                    leading_colon: None,
                    segments: constraint_path,
                };

                constraints_code.insert(
                    display_order,
                    quote! {
                       constraints.push(#constraint);
                    },
                );
                //TODO: fix styling here to use styles specified in config file
                let mut state_styling_code = TokenStream2::new();
                if stateful {
                    state_styling_code = quote! {
                        // field is selected
                        if state.selected_field.value == #display_order && state.editing_selected_field.is_some(){
                            #field_block_border_style_name = #field_block_border_style_name.cyan();
                        } else if state.selected_field.value == #display_order && state.editing_selected_field.is_none() {

                            #field_block_border_style_name = #field_block_border_style_name.red();
                        }
                    }
                }

                // special casing for other widgets
                //
                // Dropdown is always stateful
                //
                // TODO: should instead error on not stateful
                if widget_type == format_ident!("Dropdown") && stateful {
                    if widget_state.is_none() {
                        return Err(syn::Error::new_spanned(f, "No widget_state specified"));
                    }
                    if widget_options.is_empty() {
                        return Err(syn::Error::new_spanned(f, "No widget options specified"));
                    }
                    let state_struct_ident = format_ident!("state");
                    #[allow(clippy::unwrap_used)] // already checked for None above
                    let widget_state_unwrapped = widget_state.unwrap();
                    let widget_state_name = quote!(#state_struct_ident.#widget_state_unwrapped);

                    field_display_code.insert(
                        display_order,
                        quote! {
                            let mut #field_block_style_name = ratatui::style::Style::default();
                            let mut #field_block_border_style_name = ratatui::style::Style::default();
                            #state_styling_code
                            let #block_name = ratatui::widgets::block::Block::default()
                               .borders(ratatui::widgets::Borders::ALL)
                               .border_style(#field_block_border_style_name)
                               .style(#field_block_style_name)
                               .title(#field_title);
                            let mut #field_text_style_name = ratatui::style::Style::default();
                            let mut dropdown = Dropdown::new();
                            let entries = vec![#(#widget_options.to_string()),*];
                            dropdown.add_entries(entries);
                            #widget_state_name.num_entries.value = dropdown.len();
                            dropdown.block(#block_name);


                            dropdown.render_ref(layout[#display_order], buf, &mut #widget_state_name);
                        },
                    );
                } else {
                    // widget_type == default_widget_type or not stateful
                    //
                    // reset widget type to default
                    widget_type = default_widget_type;
                    //TODO: this is where to fix the widget_type issues
                    let paragraph_name_code = if is_option(&f.ty) {
                        quote! {
                            let field_value = self.#field_name.to_owned().unwrap_or_default().to_string();
                            let #paragraph_name = ratatui::widgets::#widget_type::new(
                                ratatui::text::Text::styled(
                                    field_value, #field_text_style_name)).block(#block_name);
                        }
                    } else {
                        quote! {
                            let field_value = self.#field_name.to_owned().to_string();
                            let #paragraph_name = ratatui::widgets::#widget_type::new(
                                ratatui::text::Text::styled(
                                    field_value, #field_text_style_name)).block(#block_name);
                        }
                    };

                    field_display_code.insert(
                        display_order,
                        quote! {
                            let mut #field_block_style_name = ratatui::style::Style::default();
                            let mut #field_block_border_style_name = ratatui::style::Style::default();
                            #state_styling_code
                            let #block_name = ratatui::widgets::block::Block::default()
                               .borders(ratatui::widgets::Borders::ALL)
                               .border_style(#field_block_border_style_name)
                               .style(#field_block_style_name)
                               .title(#field_title);
                            let mut #field_text_style_name = ratatui::style::Style::default();
                            #paragraph_name_code
                            #paragraph_name.render(layout[#display_order], buf);
                        },
                    );
                }
                // don't need this mapping if not stateful
                #[expect(clippy::arithmetic_side_effects)]
                if stateful && !skip && !bottom_field {
                    field_enum_mapping_code.insert(
                        display_order,
                        quote! {
                             #field_enum_variant = #display_order,
                        },
                    );
                    field_offset_enum_code.insert(
                        display_order,
                        quote! {
                            #field_enum_variant = #field_offset_value,
                        },
                    );
                    // increment this after setting the enum variant, so it is set for the next
                    // field
                    // unwrap_or_default() here is ok as these are all checked for None above here
                    field_offset_value += constraint_value.unwrap_or_default();
                }
            }
        } else {
            return Err(syn::Error::new_spanned(f, "fieldname is None"));
        }
    }
    //TODO: allow an alternate method of specifing left/right field values so you can do things like
    //display `step_id`
    let left_field_content = if let Some(field_name) = &left_field {
        if let Some(lower_field_title) = &left_lower_field_title {
            if lower_field_title.is_empty() {
                return Err(syn::Error::new_spanned(
                    left_field,
                    "`field_title` attribute specified on field with `left_field` attribute cannot be empty",
                ));
            }
            quote! {
               let left_block = ratatui::widgets::block::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .style(ratatui::style::Style::default())
                    .title(#lower_field_title);

                let left_paragraph = ratatui::widgets::Paragraph::new(
                    ratatui::text::Text::styled(
                        self.#field_name.len().to_string(),
                        ratatui::style::Style::default().fg(ratatui::style::Color::Green),
                ))
                .block(left_block);
                left_paragraph.render(left_info_area, buf);
            }
        } else {
            return Err(syn::Error::new_spanned(
                left_field,
                "`field_title` attribute needs to be specified on field with `left_field` attribute",
            ));
        }
    } else {
        quote! {
            // render an empty block with borders on the left
            let left_empty_block = ratatui::widgets::block::Block::default().borders(ratatui::widgets::Borders::ALL);
            left_empty_block.render(left_info_area, buf);
        }
    };
    let right_field_content = if let Some(field_name) = &right_field {
        if let Some(lower_field_title) = &right_lower_field_title {
            if lower_field_title.is_empty() {
                return Err(syn::Error::new_spanned(
                    right_field,
                    "`field_title` attribute specified on field with `left_field` attribute cannot be empty",
                ));
            }
            quote! {
               let right_block = ratatui::widgets::block::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .style(ratatui::style::Style::default())
                    .title(#lower_field_title);

                let right_paragraph = ratatui::widgets::Paragraph::new(
                    ratatui::text::Text::styled(
                        self.#field_name.len().to_string(),
                        ratatui::style::Style::default().fg(
                            ratatui::style::Color::Green),
                ))
                .block(right_block);
                right_paragraph.render(right_info_area, buf);
            }
        } else {
            return Err(syn::Error::new_spanned(
                right_field,
                "`field_title` attribute needs to be specified on field with `left_field` attribute",
            ));
        }
    } else {
        quote! {
            // render an empty block with borders on the right
            let right_empty_block = ratatui::widgets::block::Block::default().borders(ratatui::widgets::Borders::ALL);
            right_empty_block.render(right_info_area, buf);
        }
    };
    // add 2 for borders and 3 for bottom blocks
    total_field_height += 5;
    let constraint_code_values: Vec<TokenStream2> = constraints_code.values().cloned().collect();

    let field_display_code_values: Vec<TokenStream2> = field_display_code.values().cloned().collect();

    let field_enum_mapping_code_values: Vec<TokenStream2> = field_enum_mapping_code.values().cloned().collect();

    let field_offset_enum_code_values: Vec<TokenStream2> = field_offset_enum_code.values().cloned().collect();

    let total_field_height_value = total_field_height.0;

    let inner_fn_code = quote! {

        #len_check_fn_code
        // Use split here, since we don't care about naming the fields specifically

        //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
        //function if too many fields

        let mut constraints = Vec::new();
        if area.height >= #total_field_height_value {
            // output constraint vector pushes
           #(#constraint_code_values)*
        } else {
            //TODO: implement scrolling
            todo!("Scrolling not implemented yet. Screen Height too small")
        }
        // last constraint for step/equipment block
        constraints.push(ratatui::layout::Constraint::Length(3));

        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(constraints)
            .split(area);

        #(#field_display_code_values)*

        // recipe_edit_layout should always have something in it.
        // This is a valid place to panic
        #[allow(clippy::expect_used)]
        let [left_info_area, right_info_area] = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([ratatui::layout::Constraint::Percentage(50), ratatui::layout::Constraint::Percentage(50)])
            .areas(*layout.last().expect("No edit areas defined"));
        #left_field_content

        #right_field_content

    };
    if stateful {
        let state_struct_ident = format_ident!("{}", state_struct);
        let num_visible_fields = constraints_code.len();
        Ok(
            quote! {
                #[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive, ToPrimitive)]
                #[automatically_derived]
                #[repr(usize)]
                pub enum #field_enum_name {
                    #(#field_enum_mapping_code_values)*
                }

                #[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive, ToPrimitive)]
                #[automatically_derived]
                #[repr(u16)]
                pub enum #field_offset_enum_name {
                    #(#field_offset_enum_code_values)*
                }

                #[automatically_derived]
                impl ratatui::widgets::StatefulWidgetRef for #struct_name {
                    type State = #state_struct_ident;
                    fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State){
                        #inner_fn_code
                    }
                }

                #[automatically_derived]
                impl #struct_name {
                    pub const NUM_FIELDS: usize = #num_visible_fields;
                }

            }, // end of quote block
        )
    } else {
        Ok(
            quote! {
                #[automatically_derived]
                impl ratatui::widgets::WidgetRef for #struct_name {
                    fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer){
                        #inner_fn_code
                    }

                }
            }, // end of quote block
        )
    }
}

//https://stackoverflow.com/a/53571882/3342767
//fn make_ascii_titlecase(s: &mut str) {
//    if let Some(r) = s.get_mut(0..1) {
//        r.make_ascii_uppercase();
//    }
//}

//https://stackoverflow.com/a/53571882/3342767
/// [`to_ascii_titlecase`] outputs the input &str as titlecase
fn to_ascii_titlecase(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(r) = out.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    out
}

/// [`to_camelcase_from_snake_case`] converts a string from `snake_case` into `UpperCamelCase` for
/// use in naming enum variants. Assumes its input is `snake_case` and ASCII. Does almost no
/// validation as it is intended to be used within this derive macro only.
fn to_camelcase_from_snake_case(s: &str) -> String {
    s.split('_').map(to_ascii_titlecase).collect::<String>()
}
//https://stackoverflow.com/a/56264023/3342767
/// [`is_option`] checks if a [`syn::Type`] is `Option<T>` rather than `T`. It checks for the
/// following variants of Option. It is not exhaustive and may fail with unusual `Option`s or
/// methods of specifiying them. Stick to the standards and it will work.
fn is_option(ty: &syn::Type) -> bool {
    match ty {
        // type_path.qself is Some() if this is a self path which we do not want
        Type::Path(ref type_path) if type_path.qself.is_none() => type_path
            .path
            .segments
            .iter()
            .any(|test_str| test_str.ident.to_string().as_str() == "Option"),
        _ => false,
    }
}
