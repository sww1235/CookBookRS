//! Procedural Macros for the [`CookbookRS`] crate

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Expr, Fields, Lit, Meta};

use std::collections::BTreeMap;

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
#[proc_macro_derive(
    Widget,
    attributes(
        state_struct,
        display_order,
        constraint_value,
        constraint_type,
        display_widget,
        left_field,
        right_field,
        field_title,
        skip
    )
)]
pub fn stateful_widget_ref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_stateful_widget(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
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
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "This derive macro only works on structs with named fields.",
            ))
        }
    };

    let st_name = &input.ident;

    let mut constraints_code = BTreeMap::new();
    let mut field_display_code = BTreeMap::new();

    let mut total_field_height: u16 = 0;
    let mut left_field = None;
    let mut right_field = None;

    // allowing this, since I want it to be clear I am selecting the 0th element in the list.
    // essentially treating the iterator the same as a tuple or vec.
    // TODO: maybe figure out how to not have `state_struct` be an iterator
    #[allow(clippy::iter_nth_zero)]
    let state_struct = &input
        .attrs
        .clone()
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("state_struct") {
                if let Meta::NameValue(ref name_value) = attr.meta {
                    if let Expr::Lit(ref lit) = name_value.value {
                        if let Lit::Str(ref lit_str) = lit.lit {
                            Some(lit_str.value())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .nth(0);
    if state_struct.is_none() {
        return Err(syn::Error::new_spanned(
            &input,
            "No `state_struct` specified during `StatefulWidgetRef` derive.",
        ));
    }
    //TODO: need to fix styling here
    for f in fields {
        // want to check only if the ident of the attr is skip.
        let mut skip = f.attrs.iter().filter_map(|attr| {
            if let Meta::Path(ref path) = attr.meta {
                path.is_ident("skip").then_some(true)
            } else {
                None
            }
        });

        // allowing this, since I want it to be clear I am selecting the 0th element in the list.
        // essentially treating the iterator the same as a tuple or vec.
        // TODO: maybe figure out how to not have `skip` be an iterator
        #[allow(clippy::iter_nth_zero)]
        if skip.nth(0).unwrap_or(false) {
            continue;
        }
        if let Some(field_name) = f.ident.clone() {
            if left_field.is_some() {
                return Err(syn::Error::new_spanned(
                                f,
                                "The `left_field` attribute was specified more than once. It should only be specified on one field",
                            ));
            }
            if right_field.is_some() {
                return Err(syn::Error::new_spanned(
                                f,
                                "The `right_field` attribute was specified more than once. It should only be specified on one field",
                            ));
            }
            let block_name = format_ident!("{}_block", field_name);
            let paragraph_name = format_ident!("{}_paragraph", field_name);
            let style_name = format_ident!("{}_style", field_name);
            let field_title = format_ident!("{}_style", to_ascii_titlecase(stringify!(field_name)));
            let mut display_order = 0;
            let mut constraint_type = String::new();
            let mut constraint_value = 0;
            //this is the default widget for displaying text
            let mut widget_type = "Paragraph".to_string();
            let mut lower_field_title: Option<String> = None;

            // handle remaining attributes
            for attr in &f.attrs {
                if attr.path().is_ident("display_order") {
                    if let Meta::NameValue(ref name_value) = attr.meta {
                        if let Expr::Lit(ref lit) = name_value.value {
                            if let Lit::Int(ref lit_int) = lit.lit {
                                display_order = lit_int.base10_parse::<u16>()?;
                            } else {
                                return Err(syn::Error::new_spanned(
                                attr,
                                "The `display_order` attribute needs to be set equal to an integer",
                            ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "The `display_order` attribute needs to be set equal to an integer",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                                attr,
                                "The `display_order` attribute needs to be called as a NamedValue attribute type",
                            ));
                    }
                } else if attr.path().is_ident("constraint_type") {
                    if let Meta::NameValue(ref name_value) = attr.meta {
                        if let Expr::Lit(ref lit) = name_value.value {
                            if let Lit::Str(ref lit_str) = lit.lit {
                                match lit_str.value().as_str() {
                                    "Min" | "min" => constraint_type = "Min".to_string(),
                                    "Max" | "max" => constraint_type = "Max".to_string(),
                                    "Length" | "length" => constraint_type = "Length".to_string(),
                                    "Percentage" | "percentage" => {
                                        constraint_type = "Percentage".to_string();
                                    }
                                    "Fill" | "fill" => constraint_type = "Fill".to_string(),
                                    "Ratio" | "ratio" => {
                                        return Err(syn::Error::new_spanned(
                                                attr,
                                                "Ratio constraint type in attribute `constraint_type` is not supported by this derive macro"));
                                    }
                                    x => {
                                        let err_string =
                                            format!("Constraint type {x} is not recognized");
                                        return Err(syn::Error::new_spanned(attr, err_string));
                                    }
                                }
                            } else {
                                return Err(syn::Error::new_spanned(
                                attr,
                                "the `constraint_type` attribute needs to be set equal to a string",
                            ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "the `constraint_type` attribute needs to be set equal to a string",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                                attr,
                                "The `constraint_type` attribute needs to be called as a NamedValue attribute type",
                            ));
                    }
                } else if attr.path().is_ident("constraint_value") {
                    if let Meta::NameValue(ref name_value) = attr.meta {
                        if let Expr::Lit(ref lit) = name_value.value {
                            if let Lit::Int(ref lit_int) = lit.lit {
                                constraint_value = lit_int.base10_parse::<u16>()?;
                            } else {
                                return Err(syn::Error::new_spanned(
                                attr,
                                "The `constraint_value` attribute needs to be set equal to an integer",
                            ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "The `constraint_value` attribute needs to be set equal to an integer",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                                attr,
                                "The `constraint_value` attribute needs to be called as a NamedValue attribute type",
                            ));
                    }
                } else if attr.path().is_ident("display_widget") {
                    if let Meta::NameValue(ref name_value) = attr.meta {
                        if let Expr::Lit(ref lit) = name_value.value {
                            if let Lit::Str(ref lit_str) = lit.lit {
                                //TODO: perform validation here
                                widget_type = lit_str.value();
                            } else {
                                return Err(syn::Error::new_spanned(
                                attr,
                                "the `display_widget` attribute needs to be set equal to a string",
                            ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "the `display_widget` attribute needs to be set equal to a string",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                                attr,
                                "The `display_widget` attribute needs to be called as a NamedValue attribute type",
                            ));
                    }
                } else if attr.path().is_ident("left_field") {
                    // checking to make sure this attr is a path and doesn't have any values
                    // associated with it
                    // TODO: make sure the value of the field is a vector
                    if std::mem::discriminant(&attr.meta)
                        == std::mem::discriminant(&syn::Meta::Path(syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::new(),
                        }))
                    {
                        left_field = Some(
                            quote! {}, // empty tokenstring here for now. Need to have the
                                       // #field_title = attribute value available as well which is
                                       // not available until after the end of the for loop.
                        );
                    } else {
                        return Err(syn::Error::new_spanned(
                            attr.path(),
                            "The `left_field` attribute should not be called with a value",
                        ));
                    }
                } else if attr.path().is_ident("right_field") {
                    // checking to make sure this attr is a path and doesn't have any values
                    // associated with it
                    // TODO: make sure the value of the field is a vector
                    if std::mem::discriminant(&attr.meta)
                        == std::mem::discriminant(&syn::Meta::Path(syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::new(),
                        }))
                    {
                        right_field = Some(
                            quote! {}, // empty tokenstring here for now. Need to have the
                                       // #field_title = attribute value available as well which is
                                       // not available until after the end of the for loop.
                        );
                    } else {
                        return Err(syn::Error::new_spanned(
                            attr.path(),
                            "The `right_field` attribute should not be called with a value",
                        ));
                    }
                } else if attr.path().is_ident("field_title") {
                    if let Meta::NameValue(ref name_value) = attr.meta {
                        if let Expr::Lit(ref lit) = name_value.value {
                            if let Lit::Str(ref lit_str) = lit.lit {
                                //TODO: perform validation here
                                lower_field_title = Some(lit_str.value());
                            } else {
                                return Err(syn::Error::new_spanned(
                                    attr,
                                    "the `field_title` attribute needs to be set equal to a string",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "the `field_title` attribute needs to be set equal to a string",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                                attr,
                                "The `field_title` attribute needs to be called as a NamedValue attribute type",
                            ));
                    }
                } else {
                    continue;
                }
            }

            if left_field.is_some() {
                // this is per field
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
                left_field = Some(
                    quote! {
                       let left_block = Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .title(stringify!(lower_field_title));

                        let left_paragraph = Paragraph::new(Text::styled(
                            self.#field_name.len().to_string(),
                            Style::default().fg(Color::Green),
                        ))
                        .block(left_block);
                        left_paragraph.render(left_info_area, buf);
                    }, // end of quote block
                );
            } else {
                left_field = Some(
                    quote! {
                        // render an empty block with borders on the left
                        Widget::render(Block::default().borders(Borders::ALL), left_info_area, buf);
                    }, // end of quote block
                );
            }
            if right_field.is_some() {
                // this is per field
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
                right_field = Some(
                    quote! {
                       let right_block = Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .title(stringify!(lower_field_title));

                        let right_paragraph = Paragraph::new(Text::styled(
                            self.#field_name.len().to_string(),
                            Style::default().fg(Color::Green),
                        ))
                        .block(right_block);
                        right_paragraph.render(right_info_area, buf);
                    }, // end of quote block
                );
            } else {
                right_field = Some(
                    quote! {
                        // render an empty block with borders on the right
                        Widget::render(Block::default().borders(Borders::ALL), right_info_area, buf);
                    }, // end of quote block
                );
            }

            total_field_height += constraint_value;
            let constraint = format_ident!(
                "ratatui::layout::Constraint::{}({})",
                constraint_type,
                constraint_value
            );

            constraints_code.insert(
                display_order,
                quote! {
                   constraints.push(#constraint);
                },
            );

            field_display_code.insert(
                display_order,
                quote! {
                    let #block_name = ratatui::widgets::Block::default()
                       .borders(ratatui::Widgets::Borders::ALL)
                       .style(ratatui::style::Style::default())
                       .title(#field_title);
                    let mut #style_name = ratatui::style::Style::default();
                    // field is selected
                    if state.selected_field.0 == #display_order {
                        #style_name = #style_name.fg(ratatui::style::Color::Red);
                    }
                    let #paragraph_name = ratatui::widget::#widget_type::new(ratatui::text::Text::styled(
                            self.#field_name.clone(), #style_name)).block(#block_name);
                    #paragraph_name.render(constraints[#display_order], buf);
                },
            );
        } else {
            return Err(syn::Error::new_spanned(f, "fieldname is None"));
        }
    }
    // add 2 for borders and 3 for bottom blocks
    total_field_height += 5;
    let constraint_code_values: Vec<TokenStream2> = constraints_code.values().cloned().collect();

    let field_display_code_values: Vec<TokenStream2> =
        field_display_code.values().cloned().collect();
    Ok(
        quote! {
            #[automatically_derived]
            impl ratatui::widgets::StatefulWidgetRef for #st_name {
                type State = #state_struct;
                fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State){
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
                    #left_field

                    #right_field

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