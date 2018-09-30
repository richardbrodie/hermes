/////////////////////////////
// other callback handlers //
/////////////////////////////

function update_feed_handler(obj) {
  var new_feeds = this.state.feeds_data.filter(f => f.id != obj.feed_id);
  new_feeds.push(obj.feed)
  new_feeds.sort((f1, f2) => {
    var nameA = f1.title.toUpperCase();
    var nameB = f2.title.toUpperCase();
    if (nameA < nameB) { return -1; }
    if (nameA > nameB) { return 1; }
    return 0
  })
  this.setState({ feeds_data: new_feeds })
}

function add_new_items_handler(obj) {
  var new_items = this.state.items_data;
  new_items = obj.items.concat(new_items);
  this.setState({ items_data: new_items })
}
export { update_feed_handler, add_new_items_handler }
