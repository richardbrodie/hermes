import React, { Component } from 'react';
import { Route, Switch } from 'react-router-dom';
import Sockette from 'sockette';

import AddFeed from '../AddFeed'
import ItemList from '../ItemList'
import SingleItem from '../SingleItem'
import Sidebar from '../Sidebar/Sidebar'
import Settings from '../Settings'
import * as store from '../local_storage';
import { MainView } from './styles'

import {
  mark_item_as_read,
  select_feed_handler,
  select_item_handler,
  add_new_feed_handler,
  load_more_items_handler
} from './backend_handlers'
import { update_feed_handler, add_new_items_handler } from './other_handlers'
import { fetch_feeds, fetch_item, fetch_items } from './rest_data'


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
    this.fetch_feeds = fetch_feeds.bind(this);
    this.fetch_item = fetch_item.bind(this);
    this.fetch_items = fetch_items.bind(this);
    this.fetch_feeds();

    // backend
    this.mark_item_as_read = mark_item_as_read.bind(this)
    this.select_feed_handler = select_feed_handler.bind(this)
    this.select_item_handler = select_item_handler.bind(this)
    this.add_new_feed_handler = add_new_feed_handler.bind(this)
    this.load_more_items_handler = load_more_items_handler.bind(this)

    // frontend
    this.update_feed_handler = update_feed_handler.bind(this)
    this.add_new_items_handler = add_new_items_handler.bind(this)
  }

  render() {
    return (
      <MainView>
        <Sidebar feeds_data={this.state.feeds_data} />

        <Switch>
          <Route path="/feed/:id"
            render={(props) => <ItemList {...props} handler={this.select_feed_handler} items_data={this.state.items_data} load_more_handler={this.load_more_items_handler} />} />
          <Route path="/add" render={(props) => <AddFeed {...props} handler={this.add_new_feed_handler} />} />
          <Route path="/settings" render={(props) => <Settings {...props} />} />
          <Route path="/item/:id" render={(props) => <SingleItem {...props} handler={this.select_item_handler} item={this.state.selected_item} />} />
        </Switch>
      </MainView>
    )
  }

  ////////////////
  // websockets //
  ////////////////

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
}
