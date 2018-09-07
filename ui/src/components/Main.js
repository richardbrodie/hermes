import React, { Component } from 'react';
import { Route, Switch } from 'react-router-dom';
import Sockette from 'sockette';

import AddFeed from './AddFeed'
import ItemList from './ItemList'
import SingleItem from './SingleItem'
import Sidebar from './Sidebar'
import store from './store';

import "../styles/Main.css"

export default class Main extends Component {
  constructor(props) {
    super(props);
    // const ws = this.setup_socket();
    this.state = {
      // ws_handler: ws,
      feeds_data: new Array(),
      selected_feed_id: null,
      items_data: new Array(),
      selected_item_id: null,
      selected_item: null
    }
    this.fetch_feeds();
    this.select_feed_handler = this.select_feed_handler.bind(this)
    this.send_add_new_feed_handler = this.send_add_new_feed_handler.bind(this)
    this.mark_item_as_read = this.mark_item_as_read.bind(this)
    this.select_item_handler = this.select_item_handler.bind(this)
  }

  render() {
    return (
      <div id="main-view">
        <Sidebar feeds_data={this.state.feeds_data} />

        <Switch>
          <Route path="/feed/:id"
            render={(props) => <ItemList {...props} handler={this.select_feed_handler} items_data={this.state.items_data} />} />
          <Route path="/add" render={(props) => <AddFeed {...props} handler={this.send_add_new_feed_handler} />} />
          <Route path="/item/:id" render={(props) => <SingleItem {...props} handler={this.select_item_handler} item={this.state.selected_item} />} />
        </Switch>
      </div>
    )
  }

  // websockets
  setup_socket() {
    const url = `ws://${window.location.host}/ws?access_token=${store.currentJWT}`;
    return new Sockette(url, {
      maxAttempts: 10,
      onopen: e => console.log('Connected!', e),
      onmessage: e => {
        var data = JSON.parse(e.data)
        if (data.feed) {
          this.update_feed_handler(data)
        }
        if (data.items && this.state.selected_feed_id == data.feed_id) {
          this.add_new_items_handler(data)
        }
      },
      onreconnect: e => console.log('Reconnecting...', e),
      onmaximum: e => console.log('Stop Attempting!', e),
      onclose: e => console.log('Closed!', e),
      onerror: e => console.log('Error:', e)
    });
  }

  // REST
  fetch_feeds() {
    var url = `/api/feeds${store.accessToken}`;
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) { this.setState({ feeds_data: data }) }
      })
      .catch(error => store.msgCallback('error', error, 'warning'));
  }

  fetch_items(id) {
    var url = `/api/items/${id}${store.accessToken}`;
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) { this.setState({ items_data: data, selected_feed_id: id }); }
      })
      .catch(error => store.msgCallback('error', error, 'warning'));
  }

  fetch_item(id) {
    var url = `/api/item/${id}${store.accessToken}`;
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) {
          this.setState({ selected_item: data, selected_item_id: id });
        }
      })
      .catch(error => store.msgCallback('error', error, 'warning'));
  }

  // callback handlers
  select_feed_handler(id) {
    this.fetch_items(id)
  }

  select_item_handler(id) {
    var item = this.state.items_data.find(i => i.id == id)
    if (item) {
      item = this.mark_item_as_read(item)
      this.setState({ selected_item: item, selected_item_id: id })
    } else {
      this.fetch_item(id)
    }
  }

  update_feed_handler(obj) {
    var new_feeds = this.state.feeds_data.filter(feed => feed.id != obj.feed_id);
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

  add_new_items_handler(obj) {
    var new_items = this.state.items_data;
    new_items = obj.items.concat(new_items);
    this.setState({ items_data: new_items })
  }

  send_add_new_feed_handler(feed_url) {
    let data = { msg_type: "Subscribe", data: feed_url }
    this.state.ws_handler.send(JSON.stringify(data))
  }

  mark_item_as_read(item) {
    let new_items = this.state.items_data
    let new_feeds = this.state.feeds_data

    let item_index = new_items.indexOf(item)
    item.seen = true
    new_items.splice(item_index, 1, item)

    let feed = new_feeds.find(f => f.id == item.feed_id)
    let feed_index = new_feeds.indexOf(feed)
    feed.unseen_count--;
    new_feeds.splice(feed_index, 1, feed)

    let data = { msg_type: "MarkRead", data: item.subscribed_item_id.toString() }
    if (this.state.ws_handler) {
      this.state.ws_handler.send(JSON.stringify(data))
    }
    this.setState({ items_data: new_items, feeds_data: new_feeds })
    return item
  }
}

function make_req(url, verb) {
  var headers = new Headers({
    'Content-Type': 'application/json',
  });
  return new Request(url, {
    method: verb,
    headers: headers
  });
}

