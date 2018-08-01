import React, { Component } from 'react';
import { Route, NavLink } from 'react-router-dom';
import 'react-notifications-component/dist/theme.css'

import Topbar from './Topbar'
import Items from './Items'
import store from './store'
import "../styles/Feeds.css"

export default class Feeds extends Component {
  constructor(props) {
    super(props)
    this.state = { feeds: [] }
    this.fetchData()
  }

  render() {
    return (
      <div id="feed-view">
        <Topbar />
        <nav id='feed-list'>
          {this.state.feeds.map((feed, i) =>
            <div className="router-link" key={i}>
              <NavLink to={`/feed/${feed.id}`} > {feed.title}</NavLink>
            </div>
          )}
        </nav >
        <Route path="/feed/:id" component={Items} />
      </div>
    )
  }

  fetchData() {
    const url = "http://localhost:4000/feeds";
    var headers = new Headers({
      "Content-Type": "application/json",
      Authorization: "Bearer " + store.currentJWT
    });
    var req = new Request(url, {
      method: "GET",
      headers: headers
    });
    fetch(req)
      .then(resp => resp.json())
      .then(data => this.setState({ feeds: data }))
      .catch(error => store.msgCallback('error', error, 'warning'));
  }
}

