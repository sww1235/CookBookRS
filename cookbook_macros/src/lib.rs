//! Procedural Macros for the [`CookbookRS`] crate

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput, Expr, Fields, Lit, Meta, Token};

use std::collections::BTreeMap;

use std::mem;

///[`stateful_widget_ref_derive`] is the outer derive function that allows for
///deriving the [`StatefulWidgetRef`] trait on structs. This particular implementation generates a
///subwidget for each field in the struct not marked with the `#skip` attribute. It also generates
///a vertical [`Layout`] to display the structs
///
/// # Struct Attributes
/// - `state_struct` is the name of the struct that holds the state information for the struct that
/// [`StatefulWidgetRef`] is being derived on. this is case sensitive.
///
/// # Field Attributes
/// - `display_order` is an integer that determines the order the field will be displayed. It is
/// used as follows: `display_order = 2`.
/// - `constraint_type` is matched against the values of [`ratatui::layout::Constraint`] and
/// determines the type of constraint for each field. It supports all values except `Ratio`. It is
/// used as follows: `constraint_type = min`. The first character is not case sensitive.
/// - `constraint_value` is an integer that is used as the value inside the `Constraint`. It is
/// used as follows: `constraint_value = 5`
/// - `display_widget` is used to select the type of widget to use to display the value of the
/// field. If not specified, will default to `Paragraph`. TODO: finish this
/// - `left_field` is used to select the field that will be displayed as a count in the left hand
/// info box.
/// - `right_field` is used to select the field that will be displayed as a count in the right hand
/// info box.
/// - `skip` will skip the field from being rendered.
#[proc_macro_derive(StatefulWidgetRef, attributes(cookbook))]
pub fn stateful_widget_ref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_stateful_widget(input).unwrap_or_else(syn::Error::into_compile_error).into()
}

// TODO: maybe fix this
/// Implementation of [`StatefulWidget`] derive
#[allow(clippy::arithmetic_side_effects, clippy::too_many_lines)]
fn expand_stateful_widget(input: DeriveInput) -> syn::Result<TokenStream2> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => &fields.named,
        _ => return Err(syn::Error::new_spanned(input, "This derive macro only works on structs with named fields.")),
    };

    let st_name = &input.ident;

    let mut constraints_code = BTreeMap::new();
    let mut field_display_code = BTreeMap::new();
    let mut len_check_fn_code = TokenStream2::new();

    let mut total_field_height: u16 = 0;
    let mut left_field = None;
    let mut right_field = None;
    let mut lower_field_title: Option<String> = None;

    //This is checking outer attributes on struct, not on fields
    let state_struct = {
        let mut state_struct_value = None;
        for attr in &input.attrs {
            match &attr.meta {
                // Outer attribute will always be of form Meta::List as we are looking for
                // cookboo(__)
                // this path is the cookbook in cookbook("display_order")
                Meta::List(meta) if meta.path.is_ident("cookbook") => meta.parse_nested_meta(|inner_meta| {
                    if inner_meta.path.is_ident("state_struct") {
                        match inner_meta.value() {
                            Ok(value) => {
                                //TODO: refactor to use if-let chains once they are
                                //stablized
                                let Expr::Lit(ref lit) = value.parse()? else {
                                    return Err(inner_meta.error("The `cookbook(state_struct)` attribute must be set equal to a literal value"));
                                };
                                let Lit::Str(ref lit_str) = lit.lit else {
                                    return Err(inner_meta.error("The `cookbook(state_struct)` attribute must be set equal to a string"));
                                };
                                state_struct_value = Some(lit_str.value());
                                Ok(())
                            }
                            Err(_) => Err(inner_meta.error("The `cookbook(state_struct) attribute must be called as a NameValue attribute type")),
                        }
                    } else {
                        Ok(())
                    }
                })?,
                _ => continue,
            }
        }
        state_struct_value.ok_or(syn::Error::new_spanned(&input, "No `cookbook(state_struct)` specified during `StatefulWidgetRef` derive."))
    }?;
    //TODO: need to fix styling here
    for f in fields {
        let mut skip = false;
        //indicates if field is used to fill in a value in one of the two bottom fields
        let mut bottom_field = false;

        if let Some(field_name) = f.ident.clone() {
            // checking these right away before setting them in the next iteration of the loop
            if left_field.is_some() {
                return Err(syn::Error::new_spanned(
                    f,
                    "The `cookbooK(left_field)` attribute was specified more than once. It must only be specified on one field",
                ));
            }
            if right_field.is_some() {
                return Err(syn::Error::new_spanned(
                    f,
                    "The `cookbook(right_field)` attribute was specified more than once. It must only be specified on one field",
                ));
            }
            let block_name = format_ident!("{}_block", field_name);
            let paragraph_name = format_ident!("{}_paragraph", field_name);
            let style_name = format_ident!("{}_style", field_name);
            let field_title = format_ident!("{}_style", to_ascii_titlecase(stringify!(field_name)));
            let mut display_order: Option<u16> = None;
            let mut constraint_type: Option<String> = None;
            let mut constraint_value: Option<u16> = None;
            //this is the default widget for displaying text
            let mut widget_type = "Paragraph".to_string();

            // handle remaining attributes
            for attr in &f.attrs {
                match &attr.meta {
                    // Want outer attribute to always be a Meta::List
                    // this path is the cookbook in cookbook("display_order")
                    Meta::List(meta) if meta.path.is_ident("cookbook") => {
                        // now parse the stuff inside the parenthesis
                        meta.parse_nested_meta(|inner_meta| {
                            // #[cookbook(skip)]
                            if inner_meta.path.is_ident("skip") {
                                skip = true;
                            }

                            if inner_meta.path.is_ident("display_order") {
                                // value() advances meta.input past the = in the input. Will error
                                // if the = is not present.
                                match inner_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(inner_meta.error("The `cookbook(display_order)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Int(ref lit_int) = lit.lit else {
                                            return Err(inner_meta.error("The `cookbook(display_order)` attribute must be set equal to an integer"));
                                        };

                                        display_order = Some(lit_int.base10_parse::<u16>()?);
                                        Ok(())
                                    }
                                    Err(_) => Err(inner_meta.error("The `cookbook(display_order)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if inner_meta.path.is_ident("constraint_type") {
                                match inner_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(inner_meta.error("The `cookbook(constraint_type)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(inner_meta.error("The `cookbook(constraint_type)` attribute must be set equal to an string"));
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
                                                return Err(
                                                    inner_meta.error("Ratio constraint type in attribute `cookbook(constraint_type)` is not supported by this derive macro")
                                                );
                                            }
                                            x => {
                                                let err_string = format!("Constraint type `cookbook(constraint = {x})` is not recognized");
                                                return Err(inner_meta.error(err_string));
                                            }
                                        }
                                    }
                                    Err(_) => Err(inner_meta.error("The `cookbook(constraint_type)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if inner_meta.path.is_ident("constraint_value") {
                                match inner_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(inner_meta.error("The `cookbook(constraint_value)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Int(ref lit_int) = lit.lit else {
                                            return Err(inner_meta.error("The `cookbook(constraint_value)` attribute must be set equal to an integer"));
                                        };
                                        constraint_value = Some(lit_int.base10_parse::<u16>()?);
                                        Ok(())
                                    }

                                    Err(_) => Err(inner_meta.error("The `cookbook(constraint_value)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if inner_meta.path.is_ident("display_widget") {
                                match inner_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(inner_meta.error("The `cookbook(display_widget)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(inner_meta.error("The `cookbook(display_widget)` attribute must be set equal to an string"));
                                        };
                                        //TODO: perform validation here
                                        widget_type = lit_str.value();
                                        Ok(())
                                    }

                                    Err(_) => Err(inner_meta.error("The `cookbook(display_widget)` attribute must be called as a NameValue attribute type")),
                                }
                            } else if inner_meta.path.is_ident("left_field") {
                                // checking to make sure this field implements the iterator trait
                                // checking to make sure this attr is a path and doesn't have any values
                                // associated with it
                                // this is comparing the actual enum variant, and not the
                                // values within
                                if mem::discriminant(&attr.meta)
                                    == mem::discriminant(&Meta::Path(syn::Path {
                                        leading_colon: None,
                                        segments: Punctuated::new(),
                                    }))
                                {
                                    left_field = Some(field_name.clone());

                                    bottom_field = true;
                                    Ok(())
                                } else {
                                    return Err(inner_meta.error("The `cookbook(left_field)` attribute must not be called with a value"));
                                }
                            } else if inner_meta.path.is_ident("right_field") {
                                // checking to make sure this attr is a path and doesn't have any values
                                // associated with it
                                // this is comparing the actual enum variant, and not the
                                // values within
                                if std::mem::discriminant(&attr.meta)
                                    == std::mem::discriminant(&syn::Meta::Path(syn::Path {
                                        leading_colon: None,
                                        segments: syn::punctuated::Punctuated::new(),
                                    }))
                                {
                                    right_field = Some(field_name.clone());
                                    bottom_field = true;
                                    Ok(())
                                } else {
                                    return Err(inner_meta.error("The `cookboo(right_field)` attribute must not be called with a value"));
                                }
                            } else if inner_meta.path.is_ident("field_title") {
                                match inner_meta.value() {
                                    Ok(value) => {
                                        //TODO: refactor to use if-let chains once they are
                                        //stablized
                                        let Expr::Lit(ref lit) = value.parse()? else {
                                            return Err(inner_meta.error("The `cookbook(field_title)` attribute must be set equal to a literal value"));
                                        };
                                        let Lit::Str(ref lit_str) = lit.lit else {
                                            return Err(inner_meta.error("The `cookbook(field_title)` attribute must be set equal to an string"));
                                        };
                                        //TODO: perform validation here
                                        lower_field_title = Some(lit_str.value());
                                        Ok(())
                                    }

                                    Err(_) => Err(inner_meta.error("The `cookbook(field_title)` attribute must be called as a NameValue attribute type")),
                                }
                            } else {
                                //continue;
                                Ok(())
                            }
                        })?;
                    }
                    _ => {
                        // ignore any field attributes that are not syn::Meta::List() types with path
                        continue;
                    }
                }
            }
            if bottom_field {
                //https://users.rust-lang.org/t/derive-macro-determine-if-field-implements-trait/109417/6
                //TODO: finish this
                let field_type = &f.ty;
                let field_span = f.span();
                len_check_fn_code = quote_spanned! {field_span=>
                    fn _must_have_len_method_returning_usize(x: &#field_type)-> usize {x.len()}
                };
            }
            if skip && !bottom_field {
                continue;
            }
            //only require these fields on fields that are not skip and not iterators
            if display_order.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(f, "`the `cookbook(display_order)` attribute is not specified"));
            }
            if constraint_type.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(f, "`the `cookbook(constraint_type)` attribute is not specified"));
            }

            if constraint_value.is_none() && !bottom_field && !skip {
                return Err(syn::Error::new_spanned(f, "`the `cookbook(constraint_value)` attribute is not specified"));
            }

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

            field_display_code.insert(
                display_order,
                quote! {
                    let #block_name = ratatui::widgets::block::Block::default()
                       .borders(ratatui::widgets::Borders::ALL)
                       .style(ratatui::style::Style::default())
                       .title(#field_title);
                    let mut #style_name = ratatui::style::Style::default();
                    // field is selected
                    if state.selected_field.0 == #display_order {
                        #style_name = #style_name.fg(ratatui::style::Color::Red);
                    }
                    let #paragraph_name = ratatui::widgets::#widget_type::new(
                        ratatui::text::Text::styled(
                            self.#field_name.clone(), #style_name)).block(#block_name);
                    #paragraph_name.render(constraints[#display_order], buf);
                },
            );
        } else {
            return Err(syn::Error::new_spanned(f, "fieldname is None"));
        }
    }

    let left_field_content = if let Some(field_name) = &left_field {
        if let Some(lower_field_title) = &lower_field_title {
            if lower_field_title.is_empty() {
                return Err(syn::Error::new_spanned(
                    left_field,
                    "`field_title` attribute specified on field with `left_field` attribute cannot be empty",
                ));
            }
        } else {
            return Err(syn::Error::new_spanned(
                left_field,
                "`field_title` attribute needs to be specified on field with `left_field` attribute",
            ));
        }
        quote! {
           let left_block = ratatui::widgets::block::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .style(ratatui::style::Style::default())
                .title(stringify!(lower_field_title));

            let left_paragraph = ratatui::widgets::Paragraph::new(
                ratatui::text::Text::styled(
                    self.#field_name.len().to_string(),
                    ratatui::style::Style::default().fg(ratatui::style::Color::Green),
            ))
            .block(left_block);
            left_paragraph.render(left_info_area, buf);
        }
    } else {
        quote! {
            // render an empty block with borders on the left
            ratatui::widgets::Widget::render(
                ratatui::widgets::block::Block::default().borders(
                    ratatui::widgets::Borders::ALL), left_info_area, buf);
        }
    };
    let right_field_content = if let Some(field_name) = &right_field {
        if let Some(lower_field_title) = &lower_field_title {
            if lower_field_title.is_empty() {
                return Err(syn::Error::new_spanned(
                    right_field,
                    "`field_title` attribute specified on field with `left_field` attribute cannot be empty",
                ));
            }
        } else {
            return Err(syn::Error::new_spanned(
                right_field,
                "`field_title` attribute needs to be specified on field with `left_field` attribute",
            ));
        }
        quote! {
           let right_block = ratatui::widgets::block::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .style(ratatui::style::Style::default())
                .title(stringify!(lower_field_title));

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
        quote! {
            // render an empty block with borders on the right
            ratatui::widgets::Widget::render(
                ratatui::widgets::block::Block::default().borders(
                    ratatui::widgets::Borders::ALL), right_info_area, buf);
        }
    };
    // add 2 for borders and 3 for bottom blocks
    total_field_height += 5;
    let constraint_code_values: Vec<TokenStream2> = constraints_code.values().cloned().collect();

    let field_display_code_values: Vec<TokenStream2> = field_display_code.values().cloned().collect();
    Ok(
        quote! {
            #[automatically_derived]
            impl ratatui::widgets::StatefulWidgetRef for #st_name {
                type State = #state_struct;
                fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State){

                    #len_check_fn_code
                    // Use split here, since we don't care about naming the fields specifically

                    //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
                    //function if too many fields

                    let mut constraints = Vec::new();
                    if area.height >= #total_field_height {
                        // output constraint vector pushes
                       #(#constraint_code_values)*
                    } else {
                        //TODO: implement scrolling
                        todo!()
                    }
                    // last constraint for step/equipment block
                    constraints.push(ratatui::layout::Constraint::Length(3));

                    let layout = ratatui::layout::Layout::default()
                        .direction(ratatui::layout::Direction::Vertical)
                        .constraints(constraints)
                        .split(area);

                    #(#field_display_code_values)*

                    //TODO: add remaining 2 blocks with attributes

                    // recipe_edit_layout should always have something in it.
                    // This is a valid place to panic
                    #[allow(clippy::expect_used)]
                    let [left_info_area, right_info_area] = ratatui::layout::Layout::default()
                        .direction(ratatui::layout::Direction::Horizontal)
                        .constraints([ratatui::layout::Constraint::Percentage(50), ratatui::layout::Constraint::Percentage(50)])
                        .areas(*layout.last().expect("No edit areas defined"));
                    #left_field_content

                    #right_field_content

                }
            }

        }, // end of quote block
    )
}

//https://stackoverflow.com/a/53571882/3342767
//fn make_ascii_titlecase(s: &mut str) {
//    if let Some(r) = s.get_mut(0..1) {
//        r.make_ascii_uppercase();
//    }
//}

//https://stackoverflow.com/a/53571882/3342767
/// `to_ascii_titlecase` outputs the input &str as titlecase
fn to_ascii_titlecase(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(r) = out.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    out
}
