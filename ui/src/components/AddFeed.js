import React, { Component } from 'react';

// import * as store from './local_storage';
import { AddFeedForm, StyledContainer } from './Forms'

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

  // addFeed(feedUrl) {
  //   var url = `/api/add_feed?access_token=${store.get_jwt()}`;
  //   var body = JSON.stringify({
  //     feed_url: feedUrl
  //   });
  //   var headers = new Headers({
  //     'Content-Type': 'application/json',
  //   });
  //   var req = new Request(url, {
  //     method: 'POST',
  //     body: body,
  //     headers: headers
  //   });
  //   fetch(req)
  //     .then(resp => {
  //       if (resp.ok) {
  //         console.log("add feed success: ", `${resp.status}: ${resp.statusText}`)
  //       } else {
  //         console.log("add feed error: ", `${resp.status}: ${resp.statusText}`)
  //       }
  //     })
  //     .catch(error => console.log("add feed error: ", error));
  // }

  render() {
    return (
      <StyledContainer>
        <AddFeedForm name={"add_feed"} heading={"Add feed"} button={"add"} handler={this.handleSubmit} />
      </StyledContainer>
    );
  }
}

export default AddFeed;
