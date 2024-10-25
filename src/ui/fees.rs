use cursive::views::Dialog;
use cursive::Cursive;
use crate::api_client::get_fee_recommendations;
use tokio::runtime::Handle;

pub fn show_fee_recommendations(siv: &mut Cursive, handle: Handle) {
    // Execute the asynchronous task using the runtime handle
    let cb_sink = siv.cb_sink().clone();
    handle.spawn(async move {
        match get_fee_recommendations().await {
            Ok(fees) => {
                let fee_info = format!(
                    "Recommended Fees:\n\
                    - Fastest Confirmation: {} sat/vByte\n\
                    - Confirm in 30 minutes: {} sat/vByte\n\
                    - Confirm in 1 hour: {} sat/vByte\n\
                    - Economy Fee: {} sat/vByte\n\
                    - Minimum Fee: {} sat/vByte\n\
                    - Expulsion Threshold: {} sat/vByte",
                    fees.fastestFee,
                    fees.halfHourFee,
                    fees.hourFee,
                    fees.economyFee,
                    fees.minimumFee,
                    fees.expulsion_threshold
                );
                cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.add_layer(Dialog::info(fee_info));
                })).unwrap();
            },
            Err(e) => {
                let error_message = format!("Error fetching fees: {}", e);
                cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.add_layer(Dialog::info(error_message));
                })).unwrap();
            },
        }
    });
}

