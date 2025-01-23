mod error;
mod imp;
mod is_active;
mod subtitle;
mod title;
mod tooltip;
pub mod value;

pub use error::Error;
pub use value::Value;

use crate::profile::Profile;
use gtk::{
    gio::TlsCertificate,
    glib::{self, Object},
};
use std::rc::Rc;

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

// C-type property `value` conversion for `Item`
// * values > 0 reserved for `profile_identity_id`
const G_VALUE_GENERATE_PEM: i64 = 0;
const G_VALUE_IMPORT_PEM: i64 = -1;
const G_VALUE_GUEST_SESSION: i64 = -2;

impl Item {
    // Constructors

    pub fn new_guest_session() -> Self {
        Object::builder()
            .property("value", G_VALUE_GUEST_SESSION)
            .property("title", "Guest session")
            .property("subtitle", "No identity for this request")
            .build()
    }

    pub fn new_generate_pem() -> Self {
        Object::builder()
            .property("value", G_VALUE_GENERATE_PEM)
            .property("title", "Create new")
            .property("subtitle", "Generate long-term certificate")
            .build()
    }

    pub fn new_import_pem() -> Self {
        Object::builder()
            .property("value", G_VALUE_IMPORT_PEM)
            .property("title", "Import identity")
            .property("subtitle", "Use existing certificate")
            .build()
    }

    pub fn new_profile_identity_id(
        profile: &Rc<Profile>,
        profile_identity_id: i64,
        auth_url: &str,
    ) -> Result<Self, Error> {
        // Get PEM by ID
        match profile.identity.memory.get(profile_identity_id) {
            // Extract certificate details from PEM string
            Ok(ref pem) => match TlsCertificate::from_pem(pem) {
                // Collect certificate scopes for item
                Ok(ref certificate) => match scope(profile, profile_identity_id) {
                    // Ready to build `Item` GObject
                    Ok(ref scope) => Ok(Object::builder()
                        .property("value", profile_identity_id)
                        .property("title", title::new_for_profile_identity_id(certificate))
                        .property(
                            "subtitle",
                            subtitle::new_for_profile_identity_id(certificate, scope),
                        )
                        .property(
                            "tooltip",
                            tooltip::new_for_profile_identity_id(certificate, scope),
                        )
                        .property(
                            "is-active",
                            is_active::new_for_profile_identity_id(
                                profile,
                                profile_identity_id,
                                auth_url,
                            ),
                        )
                        .build()),
                    Err(_) => todo!(),
                },
                Err(e) => Err(Error::TlsCertificate(e)),
            },
            Err(_) => todo!(),
        }
    }

    // Actions

    /// Update properties for `Self` for given `Profile` and `auth_url`
    pub fn update(&self, profile: &Rc<Profile>, auth_url: &str) -> Result<(), Error> {
        // Update item depending on value type
        match self.value_enum() {
            Value::ProfileIdentityId(profile_identity_id) => {
                // Get PEM by ID
                match profile.identity.memory.get(profile_identity_id) {
                    // Extract certificate details from PEM string
                    Ok(ref pem) => match TlsCertificate::from_pem(pem) {
                        Ok(ref certificate) => {
                            // Get current scope
                            let scope = &scope(profile, profile_identity_id)?;

                            // Update properties
                            self.set_title(title::new_for_profile_identity_id(certificate));

                            self.set_subtitle(subtitle::new_for_profile_identity_id(
                                certificate,
                                scope,
                            ));

                            self.set_tooltip(tooltip::new_for_profile_identity_id(
                                certificate,
                                scope,
                            ));

                            self.set_is_active(is_active::new_for_profile_identity_id(
                                profile,
                                profile_identity_id,
                                auth_url,
                            ));

                            // @TODO emit update request
                        }
                        Err(e) => return Err(Error::TlsCertificate(e)),
                    },
                    Err(_) => todo!(),
                }
            }
            _ => {
                // nothing to update yet..
            }
        }
        Ok(()) // @TODO
    }

    // Getters

    /// Get `Self` C-value as `Value`
    pub fn value_enum(&self) -> Value {
        match self.value() {
            G_VALUE_GENERATE_PEM => Value::GeneratePem,
            G_VALUE_GUEST_SESSION => Value::GuestSession,
            G_VALUE_IMPORT_PEM => Value::ImportPem,
            value => Value::ProfileIdentityId(value),
        }
    }
}

// Tools

/// Collect certificate scope vector from `Profile` database for `profile_identity_id`
fn scope(profile: &Rc<Profile>, profile_identity_id: i64) -> Result<Vec<String>, Error> {
    match profile.identity.auth.database.records_scope(None) {
        Ok(result) => {
            let mut scope = Vec::new();
            for auth in result
                .iter()
                .filter(|this| this.profile_identity_id == profile_identity_id)
            {
                scope.push(auth.scope.clone())
            }
            Ok(scope)
        }
        Err(_) => todo!(),
    }
}
