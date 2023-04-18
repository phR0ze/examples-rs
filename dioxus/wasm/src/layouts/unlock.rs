use dioxus::prelude::*;
use kit::elements::{
    button::Button,
    input::{Input, Options, Validation},
    label::LabelWithEllipsis,
};
use std::default::Default;

use kit::icons::outline::Shape as Icon;

// serve as a sort of router while the user logs in]
#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq)]
pub enum AuthPages {
    Unlock,
    CreateAccount,
    Success,
}

enum UnlockError {
    ValidationError,
    InvalidPin,
    Unknown,
}

impl UnlockError {
    fn as_str(&self) -> &'static str {
        match self {
            UnlockError::ValidationError => "Something is wrong with the pin you supplied.",
            UnlockError::InvalidPin => "Hmm, that pin didn't work.",
            UnlockError::Unknown => "An unknown error occurred.",
        }
    }
}

// todo: go to the auth page if no account has been created
#[inline_props]
#[allow(non_snake_case)]
pub fn UnlockLayout(cx: Scope, page: UseState<AuthPages>, pin: UseRef<String>) -> Element {
    let validation_failure: &UseState<Option<UnlockError>> = use_state(cx, || Some(UnlockError::ValidationError)); // By default no pin is an invalid pin.

    let error: &UseState<Option<UnlockError>> = use_state(cx, || None);
    let shown_error = use_state(cx, || "");

    let account_exists: &UseState<Option<bool>> = use_state(cx, || None);
    let cmd_in_progress = use_state(cx, || false);

    // Set up validation options for the input field
    let pin_validation = Validation {
        // The input should have a maximum length of 32
        max_length: Some(32),
        // The input should have a minimum length of 4
        min_length: Some(4),
        // The input should only contain alphanumeric characters
        alpha_numeric_only: false,
        // The input should not contain any whitespace
        no_whitespace: true,
        // The input component validation is shared - if you need to allow just colons in, set this to true
        ignore_colons: false,
        // The input should allow any special characters
        // if you need special chars, select action to allow or block and pass a vec! with each char necessary, mainly if alpha_numeric_only is true
        special_chars: None,
    };

    let loading = account_exists.current().is_none();

    let image_path = crate::CONFIG
        .assets_path
        .join("images")
        .join("misc")
        .join("idle_alt.png")
        .to_str()
        .map(|x| x.to_string())
        .unwrap_or_default();

    cx.render(rsx!(
        //style {update_theme_colors(&state.current())},
        div {
            id: "unlock-layout",
            aria_label: "unlock-layout",
            if loading {
                rsx!(
                    div {
                        class: "skeletal-bars",
                        div {
                            class: "skeletal skeletal-bar",
                        },
                    }
                )
            } else {
                rsx! (
                    // img {
                    //     class: "idle",
                    //     src: "{image_path}"
                    // },
                    Input {
                        id: "unlock-input".to_owned(),
                        focus: true,
                        is_password: true,
                        icon: Icon::Key,
                        disable_onblur: !account_exists.current().unwrap_or(true),
                        aria_label: "pin-input".into(),
                        disabled: loading,
                        placeholder: "unlock.Enter Pin".into(),
                        options: Options {
                            with_validation: Some(pin_validation),
                            with_clear_btn: true,
                            with_label: Some("Welcome back, poobear".into()),
                            ellipsis_on_label: Some(LabelWithEllipsis {
                                apply_ellipsis: true,
                                padding_rigth_for_ellipsis: 105,
                            }),
                            ..Default::default()
                        }
                        onchange: move |(val, validation_passed): (String, bool)| {
                            *pin.write_silent() = val.clone();
                            // Reset the error when the person changes the pin
                            if !shown_error.get().is_empty() {
                                shown_error.set("");
                            }
                            if validation_passed {
                                cmd_in_progress.set(true);
                                validation_failure.set(None);
                            } else {
                                validation_failure.set(Some(UnlockError::ValidationError));
                            }
                        }
                        onreturn: move |_| {
                            if let Some(validation_error) = validation_failure.get() {
                                shown_error.set(validation_error.as_str());
                            } else if let Some(e) = error.get() {
                                shown_error.set(e.as_str());
                            } else {
                                page.set(AuthPages::CreateAccount);
                            }
                        }
                    },
                    (!shown_error.get().is_empty()).then(|| rsx!(
                        span {
                            class: "error",
                            "{shown_error}"
                        }
                    )),
                    div {
                        class: "unlock-details",
                        span {
                            "this is used to encrypt all of the data Uplink stores on your computer when you're not using it so nobody can read your data"
                        }
                    }
                    Button {
                            text: match account_exists.current().unwrap_or(true) {
                                true => "Unlock Account".to_string(),
                                false => "Create Account".to_string(),
                            },
                            aria_label: "create-account-button".into(),
                            appearance: kit::elements::Appearance::Primary,
                            icon: Icon::Check,
                            disabled: validation_failure.current().is_some() || *cmd_in_progress.current(),
                            onpress: move |_| {
                                if let Some(validation_error) = validation_failure.get() {
                                    shown_error.set(validation_error.as_str());
                                } else if let Some(e) = error.get() {
                                    shown_error.set(e.as_str());
                                } else {
                                    page.set(AuthPages::CreateAccount);
                                }
                            }
                        }
                )
            }
        }
    ))
}
