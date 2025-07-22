use crate::profile::Profile;
use std::rc::Rc;

pub fn new_for_profile_identity_id(
    profile: &Rc<Profile>,
    profile_identity_id: i64,
    auth_url: &str,
) -> bool {
    profile
        .identity
        .auth
        .is_matches(auth_url, profile_identity_id) // @TODO direct call?
}
