//! Tests for the validator-manager module.

#![cfg(test)]

use crate::mock::*;
use support::assert_ok;

#[test]
fn set_validators_fails_for_user() {
    new_test_ext().execute_with(|| {
        let res = SessionManager::set_validators(Origin::signed(0), vec![]);
        assert_eq!(res, Err("RequireRootOrigin"));
    })
}

#[test]
fn set_validators_works_for_root() {
    new_test_ext().execute_with(|| {
        advance_session();
        assert_eq!(Session::current_index(), 1);
        assert_eq!(Session::validators(), vec![1, 2]);

        assert_ok!(SessionManager::set_validators(Origin::ROOT, vec![1, 2, 3]));
        assert_eq!(SessionManager::validators(), vec![1, 2, 3]);

        advance_session();
        assert_eq!(Session::current_index(), 2);
        assert_eq!(Session::validators(), vec![1, 2]);

        advance_session();
        assert_eq!(Session::current_index(), 3);
        assert_eq!(Session::validators(), vec![1, 2, 3]);

        assert_ok!(SessionManager::set_validators(Origin::ROOT, vec![1, 2]));
        assert_eq!(SessionManager::validators(), vec![1, 2]);

        advance_session();
        assert_eq!(Session::current_index(), 4);
        assert_eq!(Session::validators(), vec![1, 2, 3]);

        advance_session();
        assert_eq!(Session::current_index(), 5);
        assert_eq!(Session::validators(), vec![1, 2]);

        assert_ok!(SessionManager::set_validators(Origin::ROOT, vec![1, 2, 4]));
        assert_eq!(SessionManager::validators(), vec![1, 2, 4]);

        advance_session();
        assert_eq!(Session::current_index(), 6);
        assert_eq!(Session::validators(), vec![1, 2]);

        advance_session();
        assert_eq!(Session::current_index(), 7);
        assert_eq!(Session::validators(), vec![1, 2, 4]);
    })
}
