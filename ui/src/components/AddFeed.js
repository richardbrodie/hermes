import React, { Component } from 'react';
import store from './store';
import styled from 'styled-components';

class AddFeed extends Component {
  constructor(props) {
    super(props);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    let url = formData.get('feed-url');
    this.props.handler(url)
  }

  addFeed(feedUrl) {
    var url = `/api/add_feed?access_token=${store.currentJWT}`;
    var body = JSON.stringify({
      feed_url: feedUrl
    });
    var headers = new Headers({
      'Content-Type': 'application/json',
    });
    var req = new Request(url, {
      method: 'POST',
      body: body,
      headers: headers
    });
    fetch(req)
      .then(resp => {
        if (resp.ok) {
          store.msgCallback('feed added', feedUrl, 'success');
          this.simpleDialog.hide();
        } else {
          store.msgCallback(
            'error',
            `${resp.status}: ${resp.statusText}`,
            'warning'
          );
        }
      })
      .catch(error => {
        store.msgCallback('error', error, 'warning');
      });
  }

  render() {
    return (
      <StyledAddFeed>
        <h2>Add feed</h2>
        <form onSubmit={this.handleSubmit}>
          <input name="feed-url" type="text" placeholder="feed url" />
          <button>add</button>
        </form>
      </StyledAddFeed>
    );
  }
}

export const StyledAddFeed = styled.div`
  padding-left: 0.5em;
`;

export default AddFeed;
