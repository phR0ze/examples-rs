use dioxus::prelude::*;

#[derive(Props)]
pub struct ColumnsProps<'a> {
    #[props(default)]
    is_mobile: bool,

    #[props(default)]
    is_gapless: bool,

    #[props(default)]
    is_multiline: bool,

    #[props(default)]
    is_centered: bool,

    #[props(default)]
    is_vcentered: bool,

    #[props(optional)]
    variable_gap: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,
    children: Element<'a>,
}

/// Columns are powered by Flexbox to form responsive columns
///
/// ### Properties
/// * `is_mobile: bool`
/// * `is_gapless: bool`
/// * `is_multiline: bool`
/// * `is_centered: bool`
/// * `is_vcentered: bool`
/// * `variable_gap: Option<u8>`
/// * `children: Element<'a>`
pub fn Columns<'a>(cx: Scope<'a, ColumnsProps<'a>>) -> Element {
    let mut class_name = "columns".to_string();

    if cx.props.is_mobile {
        class_name += " is-mobile";
    }

    if cx.props.is_gapless {
        class_name += " is-gapless";
    }

    if cx.props.is_multiline {
        class_name += " is-multiline";
    }

    if cx.props.is_centered {
        class_name += " is-centered";
    }

    if cx.props.is_vcentered {
        class_name += " is-vcentered";
    }

    if let Some(num) = cx.props.variable_gap {
        if (0..=8).contains(&num) {
            class_name = format!("{class_name} is-variable is-{num}");
        }
    }

    // if let Some(class) = &cx.props.custom_class {
    //     class_name += class;
    // }

    cx.render(rsx! {
        div {
            class: "{class_name}",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct ColumnProps<'a> {
    #[props(default)]
    is_three_quarters: bool,

    #[props(default)]
    is_two_thirds: bool,

    #[props(default)]
    is_half: bool,

    #[props(default)]
    is_one_third: bool,

    #[props(default)]
    is_one_quarter: bool,

    #[props(default)]
    is_full: bool,

    #[props(default)]
    is_four_fifths: bool,

    #[props(default)]
    is_three_fifths: bool,

    #[props(default)]
    is_two_fifths: bool,

    #[props(default)]
    is_one_fifth: bool,

    #[props(default)]
    is_offset_three_quarters: bool,

    #[props(default)]
    is_offset_two_thirds: bool,

    #[props(default)]
    is_offset_half: bool,

    #[props(default)]
    is_offset_one_third: bool,

    #[props(default)]
    is_offset_one_quarter: bool,

    #[props(default)]
    is_offset_four_fifths: bool,

    #[props(default)]
    is_offset_three_fifths: bool,

    #[props(default)]
    is_offset_two_fifths: bool,

    #[props(default)]
    is_offset_one_fifth: bool,

    #[props(default)]
    is_mobile: bool,

    #[props(default)]
    is_narrow: bool,

    #[props(default)]
    is_narrow_mobile: bool,

    #[props(default)]
    is_narrow_tablet: bool,

    #[props(default)]
    is_narrow_touch: bool,

    #[props(default)]
    is_narrow_desktop: bool,

    #[props(default)]
    is_narrow_widescreen: bool,

    #[props(default)]
    is_narrow_fullhd: bool,

    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    offset: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,
    children: Element<'a>,
}

/// Column
///
/// ### Properties
/// * `is_mobile: bool` by default columns only work from tablet onwards. For mobile set this value to true
/// * `is_three_quarters: bool`
/// * `is_two_thirds: bool`
/// * `is_half: bool`
/// * `is_one_third: bool`
/// * `is_one_quarter: bool`
/// * `is_full: bool`
/// * `is_four_fifths: bool`
/// * `is_three_fifths: bool`
/// * `is_two_fifths: bool`
/// * `is_one_fifth: bool`
/// * `is_size: Option<u8>` take an u8 of values 1-12 as input to specify the is-NUM classes, e.g. `is-5`
pub fn Column<'a>(cx: Scope<'a, ColumnProps<'a>>) -> Element {
    let mut classes = "column".to_string();

    if cx.props.is_narrow {
        classes += " is-narrow";
    }

    // TODO: implement the rest

    if let Some(num) = cx.props.size {
        if (0..12).contains(&num) {
            classes = format!("{classes} is-{num}");
        }
    }

    if let Some(num) = cx.props.offset {
        if (0..12).contains(&num) {
            classes = format!("{classes} is-offset-{num}");
        }
    }

    // if let Some(class) = &cx.props.custom_class {
    //     class_name += class;
    // }

    cx.render(rsx! {
        div {
            class: "{classes}",
            &cx.props.children
        }
    })
}
