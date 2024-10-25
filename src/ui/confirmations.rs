// src/ui/confirmations.rs

use cursive::views::{Dialog, TextView, EditView, LinearLayout};
use cursive::view::{Nameable, Resizable}; // Import Nameable and Resizable
use cursive::Cursive;
use crate::api_client::{estimate_confirmation_time, is_tx_in_mempool};
use tokio::runtime::Handle;

pub fn show_estimate_confirmation_time(siv: &mut Cursive, handle: Handle) {
    siv.add_layer(
        Dialog::new()
            .title("Confirmation Time Estimate")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Enter fee per vByte:"))
                    .child(
                        EditView::new()
                            .on_submit(move |siv, input| {
                                let fee_per_vsize: f64 = input.parse().unwrap_or(0.0);
                                let cb_sink = siv.cb_sink().clone();
                                handle.spawn(async move {
                                    match estimate_confirmation_time(fee_per_vsize).await {
                                        Ok(estimation) => {
                                            let message = format!(
                                                "Confirmation Time Estimate:\n\
                                                - Estimated Time: {} minutes\n\
                                                - Estimated Blocks: {}",
                                                estimation.estimated_time_minutes,
                                                estimation.estimated_blocks
                                            );
                                            cb_sink
                                                .send(Box::new(move |siv: &mut Cursive| {
                                                    siv.pop_layer();
                                                    siv.add_layer(Dialog::info(message));
                                                }))
                                                .unwrap();
                                        }
                                        Err(e) => {
                                            let error_message = format!("Error: {}", e);
                                            cb_sink
                                                .send(Box::new(move |siv: &mut Cursive| {
                                                    siv.add_layer(Dialog::info(error_message));
                                                }))
                                                .unwrap();
                                        }
                                    }
                                });
                            })
                            .with_name("fee_input")
                            .fixed_width(20)
                    )
            )
            .button("Cancel", |s| {
                s.pop_layer();
            })
    );
}

pub fn check_tx_in_mempool(siv: &mut Cursive, handle: Handle) {
    siv.add_layer(
        Dialog::new()
            .title("Check TXID in Mempool")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Enter TXID:"))
                    .child(
                        EditView::new()
                            .on_submit(move |siv, input| {
                                let txid = input.to_string();
                                let cb_sink = siv.cb_sink().clone();
                                handle.spawn(async move {
                                    match is_tx_in_mempool(txid.clone()).await {
                                        Ok(in_mempool) => {
                                            let message = if in_mempool {
                                                format!("Transaction {} is in the mempool.", txid)
                                            } else {
                                                format!("Transaction {} is NOT in the mempool.", txid)
                                            };
                                            cb_sink
                                                .send(Box::new(move |siv: &mut Cursive| {
                                                    siv.pop_layer();
                                                    siv.add_layer(Dialog::info(message));
                                                }))
                                                .unwrap();
                                        }
                                        Err(e) => {
                                            let error_message = format!("Error: {}", e);
                                            cb_sink
                                                .send(Box::new(move |siv: &mut Cursive| {
                                                    siv.add_layer(Dialog::info(error_message));
                                                }))
                                                .unwrap();
                                        }
                                    }
                                });
                            })
                            .with_name("txid_input")
                            .fixed_width(64)
                    )
            )
            .button("Cancel", |s| {
                s.pop_layer();
            })
    );
}

