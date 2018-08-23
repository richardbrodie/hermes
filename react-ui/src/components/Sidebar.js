import React, { Component } from 'react';
import { NavLink, Link } from 'react-router-dom';

import store from './store';

import '../styles/Sidebar.css';

class Sidebar extends Component {
  constructor(props) {
    super(props);
    this.state = { feeds: [] };
    this.fetchData();
  }

  render() {
    return (
      <div id="sidebar">
        <div id="top-bar">
          <Link to="/">Feeds.rs</Link>
        </div>
        <nav id="feed-list">
          {this.state.feeds.map((feed, i) => (
            <NavLink to={`/feed/${feed.id}`}>
              <div className="router-link" key={i}>
                <span className="feed-title">{feed.title}</span>
                <span className="feed-count">{feed.unseen_count}</span>
              </div>
            </NavLink>
          ))}
        </nav>
        <div className="btnCtr">
          <Link className="menuItem" to="/add">
            Add feed
          </Link>
          <Link className="menuItem" to="/add">
            Settings
          </Link>
        </div>
      </div>
    );
  }

  fetchData() {
    const url = '/api/feeds';
    var headers = new Headers({
      'Content-Type': 'application/json',
      Authorization: 'Bearer ' + store.currentJWT
    });
    var req = new Request(url, {
      method: 'GET',
      headers: headers
    });
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        if (data) { this.setState({ feeds: data }) }
      })
      .catch(error => store.msgCallback('error', error, 'warning'));
  }
}

export default Sidebar;
