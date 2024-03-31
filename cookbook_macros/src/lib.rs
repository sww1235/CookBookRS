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
/// - `skip` will skip the field from being rendered.
#[proc_macro_derive(
    Widget,
    attributes(
        state_struct,
        display_order,
        constraint_value,
        constraint_type,
        display_widget,
        skip
    )
)]
pub fn stateful_widget_ref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_widget(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_widget(input: DeriveInput) -> syn::Result<TokenStream2> {
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
    for f in fields.iter() {
        // want to check only if the ident of the attr is skip.
        let mut skip = f.attrs.iter().filter_map(|attr| {
            if let Meta::Path(ref path) = attr.meta {
                if path.is_ident("skip") {
                    Some(true)
                } else {
                    None
                }
            } else {
                None
            }
        });

        if skip.nth(0).unwrap_or(false) {
            continue;
        }
        if let Some(field_name) = f.ident.clone() {
            let block_name = format_ident!("{}_block", field_name);
            let paragraph_name = format_ident!("{}_paragraph", field_name);
            let style_name = format_ident!("{}_style", field_name);
            let field_title = format_ident!("{}_style", to_ascii_titlecase(stringify!(field_name)));
            let mut display_order = 0;
            let mut constraint_type = String::new();
            let mut constraint_value = 0;
            //this is the default widget
            let mut widget_type = "Paragraph".to_string();

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
                                        constraint_type = "Percentage".to_string()
                                    }
                                    "Fill" | "fill" => constraint_type = "Fill".to_string(),
                                    "Ratio" | "ratio" => {
                                        return Err(syn::Error::new_spanned(
                                                attr,
                                                "Ratio constraint type in attribute `constraint_type` is not supported by this derive macro"));
                                    }
                                    x => {
                                        return Err(syn::Error::new_spanned(
                                            attr,
                                            "Constraint type {x} is not recognized",
                                        ));
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
                } else {
                    continue;
                }
            }
            total_field_height += constraint_value;
            let constraint = format_ident!(
                "ratatui::layout::Constraint::{}({})",
                constraint_type,
                constraint_value
            );
            //let widget_type_ident = format_ident!("ratatui::widget::{}::new")

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
                //);
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
    Ok(quote! {
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

                let edit_layout = ratatui::layout::Layout::default()
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
                    .areas(*edit_layout.last().expect("No edit areas defined"));
            }
        }

    })
}

//https://stackoverflow.com/a/53571882/3342767
fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

//https://stackoverflow.com/a/53571882/3342767
fn to_ascii_titlecase(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(r) = out.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    out
}
