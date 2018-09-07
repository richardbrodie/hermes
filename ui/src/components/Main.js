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
    this.setup_socket();
  }

  render() {
    return (
      <div id="main-view">
        <Sidebar />

        <Switch>
          <Route path="/feed/:id" component={ItemList} />
          <Route path="/add" component={AddFeed} />
          <Route path="/item" render={(props) => <SingleItem {...props} />} />
        </Switch>
      </div>
    )
  }

  setup_socket() {
    const url = `ws://${window.location.hostname}/ws?access_token=${store.currentJWT}`;
    const ws = new Sockette(url, {
      maxAttempts: 10,
      onopen: e => console.log('Connected!', e),
      onmessage: e => console.log('Received:', e.data),
      onreconnect: e => console.log('Reconnecting...', e),
      onmaximum: e => console.log('Stop Attempting!', e),
      onclose: e => console.log('Closed!', e),
      onerror: e => console.log('Error:', e)
    });
  }
}

