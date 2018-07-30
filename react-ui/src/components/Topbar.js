import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import SkyLight from 'react-skylight';

import store from './store'

class TopBar extends Component {
  constructor(props) {
    super(props)
    this.state = {
      modalIsOpen: false
    };
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    let url = formData.get("feed-url")
    this.addFeed(url)
  }

  addFeed(feedUrl) {
    var url = "http://localhost:4000/add_feed";
    var body = JSON.stringify({
      feed_url: feedUrl,
    });
    var headers = new Headers({
      "Content-Type": "application/json",
      Authorization: "Bearer " + store.currentJWT
    });
    var req = new Request(url, {
      method: "POST",
      body: body,
      headers: headers
    });
    fetch(req)
      .then(resp => {
        if (resp.ok) {
          store.msgCallback('feed added', feedUrl, 'success')
          this.simpleDialog.hide()
        } else {
          store.msgCallback('error', `${resp.status}: ${resp.statusText}`, 'warning')
        }
      }).catch(error => {
        store.msgCallback('error', error, 'warning');
      });
  }

  render() {
    return (
      <div id="top-bar" >
        <div className="menuMainItem"><Link to='/' >Feeds.rs</Link></div>
        <div className="menuItem">
          <button onClick={() => this.simpleDialog.show()}>add feed</button>
        </div>
        <SkyLight hideOnOverlayClicked ref={ref => this.simpleDialog = ref} title="Add feed" >
          <form onSubmit={this.handleSubmit}>
            <input name="feed-url" type="text" placeholder="feed url" />
            <button>add</button>
          </form>
        </SkyLight>
      </div>
    )
  }
}

export default TopBar
