use crate::{Error, mock::*};
use frame_support::{assert_noop, assert_ok, traits::{OnFinalize, OnInitialize}};

fn run_to_block( n: u64) {
	while System::block_number() < n {
		TemplateModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		TemplateModule::on_initialize(System::block_number());
	}
}

#[test]
fn deposit() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let player = 1;
		let amount: u64 = 1000;
		assert_ok!(TemplateModule::deposit(Origin::signed(player), amount));
		assert_eq!(amount, TemplateModule::chip(player));
		assert_eq!(4000, Balances::free_balance(&player));
	});
}

#[test]
fn withdraw() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let player = 1;
		let amount: u64 = 1000;
		assert_ok!(TemplateModule::deposit(Origin::signed(player), amount));

		assert_ok!(TemplateModule::withdraw(Origin::signed(player), amount));
		assert_eq!(0, TemplateModule::chip(player));
		assert_eq!(5000, Balances::free_balance(&player));
	});
}


#[test]
fn withdraw_chip_not_enough() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let player = 1;
		let amount: u64 = 1000;
		assert_noop!(TemplateModule::withdraw(Origin::signed(player), amount), Error::<Test>::ChipNotEnough);
	});
}

#[test]
fn stake() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let player = 1;
		let amount: u64 = 1000;
		assert_ok!(TemplateModule::deposit(Origin::signed(player), amount));

		let stake: u64 = 100;
		assert_ok!(TemplateModule::stake(player, stake));
		assert_eq!(amount - stake, TemplateModule::chip(player));
	});
}

#[test]
fn draw() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		let player = 1;
		let amount: u64 = 1000;
		assert_ok!(TemplateModule::deposit(Origin::signed(player), amount));

		let stake: u64 = 100;
		assert_ok!(TemplateModule::stake(player, stake));
		assert_ok!(TemplateModule::draw(player, stake));
		
		assert_eq!(amount, TemplateModule::chip(player));
	});
}