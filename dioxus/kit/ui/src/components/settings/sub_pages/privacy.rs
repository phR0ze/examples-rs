use common::language::get_local_text;
use common::state::ToastNotification;
use common::{icons::outline::Shape as Icon, state::State};
use dioxus::prelude::*;
use kit::{
    components::section::Section,
    elements::{button::Button, switch::Switch, Appearance},
};
use warp::logging::tracing::log;

#[allow(non_snake_case)]
pub fn PrivacySettings(cx: Scope) -> Element {
    log::trace!("Privacy settings page rendered.");
    let state = use_shared_state::<State>(cx)?;

    cx.render(rsx!(
        div {
            id: "settings-privacy",
            aria_label: "settings-privacy",
            Section {
                section_label: get_local_text("settings-privacy.backup-recovery-phrase"),
                section_description: get_local_text("settings-privacy.backup-phrase-description"),
                Button {
                    text: get_local_text("settings-privacy.backup-phrase"),
                    aria_label: "backup-phrase-button".into(),
                    appearance: Appearance::Secondary,
                    icon: Icon::DocumentText,
                }
            },
            Section {
                section_label: "Test out toast".to_string(),
                section_description: "Flip the switch to trigger a toast".to_string(),
                Switch {
                    active: false,
                    onflipped: move |e| {
                        state
                        .write()
                        .mutate(common::state::Action::AddToastNotification(
                            ToastNotification::init(
                                "".into(),
                                "Foo foo foo".into(),
                                None,
                                2,
                            ),
                        ));
                    }
                }
            },
        }
    ))
}
