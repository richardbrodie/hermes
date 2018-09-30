///////////////////////////////
// backend callback handlers //
///////////////////////////////

function select_feed_handler(id) {
  this.setState({ items_data: [], selected_feed_id: id, last_date: null }, function () {
    this.fetch_items(id)
  })
}

function load_more_items_handler() {
  this.fetch_items(this.state.selected_feed_id)
}

function select_item_handler(id) {
  var item = this.state.items_data.find(i => i.id == id)
  if (item) {
    this.mark_item_as_read(item, id)
  } else {
    this.fetch_item(id)
  }
}

// ws
function add_new_feed_handler(feed_url) {
  let data = { msg_type: "Subscribe", feed_url: feed_url }
  this.state.ws_handler.send(JSON.stringify(data))
  this.props.history.push('/');
}

// ws
function mark_item_as_read(item, id) {
  let new_items = this.state.items_data
  let new_feeds = this.state.feeds_data

  let item_index = new_items.indexOf(item)
  item.seen = true
  new_items.splice(item_index, 1, item)
  this.setState({ selected_item: item, selected_item_id: id, items_data: new_items })

  let feed = new_feeds.find(f => f.id == item.feed_id)
  let feed_index = new_feeds.indexOf(feed)
  feed.unseen_count--;
  new_feeds.splice(feed_index, 1, feed)
  this.setState({ feeds_data: new_feeds })

  let data = { msg_type: "MarkRead", data: item.subscribed_item_id.toString() }
  if (this.state.ws_handler) {
    this.state.ws_handler.send(JSON.stringify(data))
  }
  return item
}

export {
  select_feed_handler,
  load_more_items_handler,
  select_item_handler,
  add_new_feed_handler,
  mark_item_as_read
}
