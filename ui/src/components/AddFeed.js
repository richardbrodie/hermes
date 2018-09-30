import React, { Component } from 'preact';
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

  render() {
    return (
      <StyledContainer>
        <AddFeedForm name={"add_feed"} heading={"Add feed"} button={"add"} handler={this.handleSubmit} />
      </StyledContainer>
    );
  }
}

export default AddFeed;
