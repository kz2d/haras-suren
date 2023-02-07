mod svg_to_jpeg;

use std::{collections::HashSet, path::Path, sync::Arc};

use bookbinder_common::convert_svg_to_jpg;
use teloxide::{prelude::*, types::InputFile};
use tokio::{fs, sync::Mutex};

#[tokio::main]
async fn main() {
    // let mut groups = Arc::new(Mutex::new(HashSet::new()));
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // fs::write("pattetesth.jpg", convert_svg_to_jpg((fs::read_to_string("test.svg").await?).as_str(), Some(100)).unwrap()).await?;
        fs::write(
            "asdasd.jpg",
            convert_svg_to_jpg((fs::read_to_string("test.svg").await?).as_str(), None).unwrap(),
        )
        .await?;
        // convert_svg_to_jpg((fs::read_to_string("test.svg").await?).as_str(), None).unwrap();
        bot.send_photo(
            msg.chat.id,
            InputFile::memory(
                convert_svg_to_jpg((fs::read_to_string("test.svg").await?).as_str(), None).unwrap(),
            ),
        )
        .await?; // InputFile::url(url::Url::parse("https://leetcode-stats-six.vercel.app/?username=KnlnKS").unwrap())).await?;
        Ok(())
    })
    .await;
}
