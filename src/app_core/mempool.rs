use cursive::traits::Nameable;
use cursive::view::Resizable;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use reqwest::Error;

/// Checks the status of a transaction by its txid
pub async fn check_transaction(txid: &str) -> Result<String, Error> {
    let api_url = format!("https://mempool.space/api/tx/{}/status", txid);

    let response = reqwest::get(&api_url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    if response["confirmed"].as_bool().unwrap_or(false) {
        Ok(format!("Transaction {} has already been confirmed.", txid))
    } else {
        Ok(format!(
            "Transaction {} is in the mempool and waiting for confirmation.",
            txid
        ))
    }
}

/// Displays a dialog in the Cursive TUI for users to input a txid and check its status
pub fn show_transaction_check(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();

    siv.add_layer(
        Dialog::new()
            .title("Mempool - TXID")
            .content(
                EditView::new()
                    .on_submit(move |s, txid| {
                        let txid = txid.to_string();
                        let cb_sink = cb_sink.clone();

                        tokio::spawn(async move {
                            match check_transaction(&txid).await {
                                Ok(status) => {
                                    let _ = cb_sink.send(Box::new(move |siv| {
                                        siv.add_layer(
                                            Dialog::info(status).title("Transaction Status"),
                                        );
                                    }));
                                }
                                Err(_) => {
                                    let _ = cb_sink.send(Box::new(move |siv| {
                                        siv.add_layer(Dialog::info(
                                            "Failed to fetch transaction status.",
                                        ));
                                    }));
                                }
                            }
                        });
                        s.pop_layer(); // Remove input dialog after submission
                    })
                    .with_name("txid_input")
                    .fixed_width(40),
            )
            .button("Search", |s| {
                let txid = s
                    .call_on_name("txid_input", |view: &mut EditView| view.get_content())
                    .unwrap();

                let cb_sink = s.cb_sink().clone();
                let txid = txid.to_string();

                tokio::spawn(async move {
                    match check_transaction(&txid).await {
                        Ok(status) => {
                            let _ = cb_sink.send(Box::new(move |siv| {
                                siv.add_layer(Dialog::info(status).title("Transaction Status"));
                            }));
                        }
                        Err(_) => {
                            let _ = cb_sink.send(Box::new(move |siv| {
                                siv.add_layer(Dialog::info("Failed to fetch transaction status."));
                            }));
                        }
                    }
                });
                s.pop_layer(); // Remove input dialog after submission
            })
            .button("Quit", |s| {
                s.pop_layer();
            }),
    );
}
