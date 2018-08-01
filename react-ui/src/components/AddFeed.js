import React, { Component } from 'react';

import store from './store'

class AddFeed extends Component {
  constructor(props) {
    super(props)
    this.handleSubmit = this.handleSubmit.bind(this)
  }

  handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    let url = formData.get("feed-url")
    this.addFeed(url)
  }

  addFeed(feedUrl) {
    // var url = "http://localhost:4000/add_feed";
    var url = "/add_feed";
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
      <div id="add-feed" >
        <h2>Add feed</h2>
        <form onSubmit={this.handleSubmit}>
          <input name="feed-url" type="text" placeholder="feed url" />
          <button>add</button>
        </form>
      </div>
    )
  }
}

export default AddFeed
