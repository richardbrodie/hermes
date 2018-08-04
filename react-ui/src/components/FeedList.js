import React, { Component } from 'react';
import { NavLink } from 'react-router-dom';

import store from './store'

import "../styles/FeedList.css"

class FeedList extends Component {
  constructor(props) {
    super(props)
    this.state = { feeds: [] }
    this.fetchData()
  }

  render() {
    return (
      <nav id='feed-list'>
        {this.state.feeds.map((feed, i) =>
          <div className="router-link" key={i}>
            <NavLink to={`/feed/${feed.id}`} > {feed.title}</NavLink>
          </div>
        )}
      </nav >
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

export default FeedList
