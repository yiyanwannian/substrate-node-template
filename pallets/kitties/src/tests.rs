use crate::mock::*;
use crate::mock::OwnedKittiesTest;
use crate::{OwnedKittiesList, KittyLinkedItem};

#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        OwnedKittiesList::<Test>::append(&0, 1);

        assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
            prev: Some(1),
            next: Some(1),
        }));
    });
}

#[test]
fn owned_kitties_can_remove_values() {
    new_test_ext().execute_with(|| {
        OwnedKittiesList::<Test>::append(&0, 1);
        OwnedKittiesList::<Test>::append(&0, 2);
        OwnedKittiesList::<Test>::append(&0, 3);

        OwnedKittiesList::<Test>::remove(&0, 2);

        assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
            prev: Some(3),
            next: Some(1),
        }));
    });
}

#[test]
fn owned_kitties_create() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
    });
}

#[test]
fn owned_kitties_breed() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
        assert_eq!(Kitties::breed(Origin::signed(1), 0, 1), Ok(()));
    });
}

#[test]
fn owned_kitties_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
        assert_eq!(Kitties::transfer(Origin::signed(1), 2, 0), Ok(()));
    });
}

#[test]
fn owned_kitties_ask() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
        assert_eq!(Kitties::ask(Origin::signed(1), 0, Some(100)), Ok(()));
    });
}

#[test]
fn owned_kitties_buy() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
        assert_eq!(Kitties::ask(Origin::signed(1), 0, Some(100)), Ok(()));
        assert_eq!(Kitties::buy(Origin::signed(1), 0, 110), Ok(()));
    });
}

//https://github.com/paritytech/substrate/blob/master/frame/assets/src/lib.rs#L148
