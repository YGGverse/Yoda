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
        .memory
        .match_scope(auth_url)
        .is_some_and(|auth| auth.profile_identity_id == profile_identity_id)
}
