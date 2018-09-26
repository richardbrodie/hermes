import React, { Component } from 'react';
import { Route, Switch } from 'react-router-dom';
import styled from 'styled-components';
import Sockette from 'sockette';

import AddFeed from './AddFeed'
import ItemList from './ItemList'
import SingleItem from './SingleItem'
import Sidebar from './Sidebar/Sidebar'
import Settings from './Settings'
import * as store from './local_storage';

const styled_a = `
  a {
    color: black;
    text-decoration: none;
    outline: 0;
  }
`;
const MainView = styled.div`
  ${ styled_a}
  height: 99vh;
  display: grid;
  grid-template-rows: 6vh auto;
  grid-template-columns: 230px 1fr;
`;

export default class Main extends Component {
  constructor(props) {
    super(props);
    const ws = this.setup_socket();
    this.state = {
      ws_handler: ws,
      feeds_data: new Array(),
      selected_feed_id: null,
      items_data: new Array(),
      selected_item_id: null,
      selected_item: null,
      last_date: null
    }
    this.fetch_feeds();
    this.select_feed_handler = this.select_feed_handler.bind(this)
    this.send_add_new_feed_handler = this.send_add_new_feed_handler.bind(this)
    this.mark_item_as_read = this.mark_item_as_read.bind(this)
    this.select_item_handler = this.select_item_handler.bind(this)
    this.load_more_handler = this.load_more_handler.bind(this)
  }

  render() {
    return (
      <MainView>
        <Sidebar feeds_data={this.state.feeds_data} />

        <Switch>
          <Route path="/feed/:id"
            render={(props) => <ItemList {...props} handler={this.select_feed_handler} items_data={this.state.items_data} load_more_handler={this.load_more_handler} />} />
          <Route path="/add" render={(props) => <AddFeed {...props} handler={this.send_add_new_feed_handler} />} />
          <Route path="/settings" render={(props) => <Settings {...props} />} />
          <Route path="/item/:id" render={(props) => <SingleItem {...props} handler={this.select_item_handler} item={this.state.selected_item} />} />
        </Switch>
      </MainView>
    )
  }

  // websockets
  setup_socket() {
    const url = `ws://${window.location.host}/ws${store.access_token_str()}`;
    return new Sockette(url, {
      maxAttempts: 10,
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
      onerror: e => console.log('Error:', e)
    });
  }

  // REST
  fetch_feeds() {
    var url = `/api/feeds${store.access_token_str()}`;
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) { this.setState({ feeds_data: data }) }
      })
      .catch(error => console.log("fetch feeds error: ", error));
  }

  fetch_items(id) {
    var url = `/api/items/${id}${store.access_token_str()}`;
    if (this.state.last_date) {
      url = `${url}&updated=${this.state.last_date}`;
    }
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) {
          let last_date = data[data.length - 1].published_at;
          if (this.state.last_date) {
            this.setState((prevState, _props) => ({
              last_date: last_date,
              items_data: prevState.items_data.concat(data)
            }));
          } else {
            this.setState({
              last_date: last_date,
              items_data: data
            });
          }
        }
      })
      .catch(error => console.log("fetch items error: ", error));
  }

  fetch_item(id) {
    var url = `/api/item/${id}${store.access_token_str()}`;
    var req = make_req(url, 'GET');
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) {
          this.setState({ selected_item: data, selected_item_id: id });
        }
      })
      .catch(error => console.log("fetch item error: ", error));
  }

  // callback handlers
  select_feed_handler(id) {
    this.setState({ items_data: [], selected_feed_id: id, last_date: null }, function () {
      this.fetch_items(id)
    })
  }
  load_more_handler() {
    this.fetch_items(this.state.selected_feed_id)
  }

  select_item_handler(id) {
    var item = this.state.items_data.find(i => i.id == id)
    if (item) {
      this.mark_item_as_read(item, id)
    } else {
      this.fetch_item(id)
    }
  }

  update_feed_handler(obj) {
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

  add_new_items_handler(obj) {
    var new_items = this.state.items_data;
    new_items = obj.items.concat(new_items);
    this.setState({ items_data: new_items })
  }

  send_add_new_feed_handler(feed_url) {
    let data = { msg_type: "Subscribe", data: feed_url }
    this.state.ws_handler.send(JSON.stringify(data))
    this.props.history.push('/');
  }

  mark_item_as_read(item, id) {
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

