use askama::Template;
use models::{FeedChannel, FeedItem};

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
  pub _parent: BaseTemplate,
  pub channels: &'a Vec<FeedChannel>,
}
impl<'a> IndexTemplate<'a> {
  pub fn new(channels: &'a Vec<FeedChannel>) -> IndexTemplate<'a> {
    IndexTemplate {
      _parent: BaseTemplate {},
      channels: channels,
    }
  }
}

#[derive(Template)]
#[template(path = "channel.html")]
pub struct FeedChannelTemplate<'a> {
  pub _parent: BaseTemplate,
  pub channel: &'a FeedChannel,
  pub items: &'a Vec<FeedItem>,
}
impl<'a> FeedChannelTemplate<'a> {
  pub fn new(feed: &'a (FeedChannel, Vec<FeedItem>)) -> FeedChannelTemplate<'a> {
    let (channel, items) = feed;
    FeedChannelTemplate {
      _parent: BaseTemplate {},
      channel: channel,
      items: items,
    }
  }
}
