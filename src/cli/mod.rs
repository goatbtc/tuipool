use std::error::Error;
use cursive::Cursive;
use cursive::traits::{Nameable, Resizable}; // Keep this import and remove the duplicate
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use reqwest;
use serde::Deserialize;
use tokio::runtime::Runtime;

const MEMPOOL_URL: &str = "https://mempool.space/api/mempool";
const BLOCKSTREAM_FEE_URL: &str = "https://blockstream.info/api/fee-estimates";

#[derive(Deserialize, Debug)]
struct MempoolData {
    count: u64,
    vsize: u64,
}

#[derive(Deserialize, Debug)]
struct FeeEstimates {
    #[serde(rename = "1")]
    fastest_fee: f64,
    #[serde(rename = "3")]
    three_blocks_fee: f64,
    #[serde(rename = "6")]
    six_blocks_fee: f64,
}

pub fn setup_mempool_menu(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();

    siv.add_layer(
        Dialog::new()
            .title("Mempool Information")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Fetching recent fee rates...").with_name("fee_view"))
                    .child(TextView::new("Enter a txid to check:"))
                    .child(
                        EditView::new()
                            .on_submit({
                                let cb_sink = cb_sink.clone();
                                move |s, txid| {
                                    let txid_clone = txid.to_string();
                                    s.pop_layer();
                                    check_txid_in_mempool(cb_sink.clone(), txid_clone);
                                }
                            })
                            .with_name("txid_input")
                            .fixed_width(40),
                    ),
            )
            .button("Refresh Fees", {
                let cb_sink = cb_sink.clone();
                move |s| {
                    s.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content("Fetching latest fee rates...");
                    });
                    fetch_and_display_fees(cb_sink.clone());
                }
            })
            .button("Close", |s| {
                s.pop_layer();
            }),
    );

    fetch_and_display_fees(cb_sink);
}

fn fetch_and_display_fees(cb_sink: cursive::CbSink) {
    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        match fetch_fee_rates().await {
            Ok(fee_data) => {
                let fee_info = format!(
                    "Fastest (1 block): {:.2} sat/vB\n3 blocks: {:.2} sat/vB\n6 blocks: {:.2} sat/vB",
                    fee_data.fastest_fee, fee_data.three_blocks_fee, fee_data.six_blocks_fee
                );
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content(fee_info);
                    });
                }));
            }
            Err(_) => {
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content("Failed to fetch fee data.");
                    });
                }));
            }
        }
    });
}

async fn fetch_fee_rates() -> Result<FeeEstimates, Box<dyn Error>> {
    let response = reqwest::get(BLOCKSTREAM_FEE_URL).await?;
    let fee_data = response.json::<FeeEstimates>().await?;
    Ok(fee_data)
}

fn check_txid_in_mempool(cb_sink: cursive::CbSink, txid: String) {
    let rt = Runtime::new().unwrap();
    let url = format!("https://mempool.space/api/tx/{}", txid);
    rt.spawn(async move {
        let client = reqwest::Client::new();
        let dialog_message = match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => "Transaction is in the mempool!".to_string(),
            _ => "Transaction not found in the mempool.".to_string(),
        };

        let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
            siv.add_layer(Dialog::info(dialog_message));
        }));
    });
}

