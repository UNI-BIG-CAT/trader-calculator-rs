use crate::constant::{ActionType, StockType};
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use rusqlite::Connection;
use rusqlite::Result;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn handle_add_position() {}

pub fn handle_reduce_position() {}

pub fn handle_back_position() {}

pub fn handle_close_position() {}
