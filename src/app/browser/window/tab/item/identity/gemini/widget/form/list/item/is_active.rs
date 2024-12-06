use crate::profile::Profile;
use std::rc::Rc;

pub fn new_for_profile_identity_gemini_id(
    profile: Rc<Profile>,
    profile_identity_gemini_id: i64,
    auth_url: &str,
) -> bool {
    profile
        .identity
        .gemini
        .auth
        .memory
        .match_scope(&auth_url)
        .is_some_and(|auth| auth.profile_identity_gemini_id == profile_identity_gemini_id)
}
