import React, { Component } from "react";
import TimeAgo from "react-timeago";

import store from "./store";

import "../styles/SingleItem.css";

class SingleItem extends Component {
  constructor(props) {
    super(props);
    this.state = { item: props.location.state.item };
    this.fetchData = this.fetchData.bind(this);
    // this.fetchData();
    console.log(
      "placeholder: mark ",
      this.state.subscribed_item_id,
      " as read"
    );
  }
  render() {
    return (
      <div id="single-item">
        <h1>
          <a href={this.state.item.link}>{this.state.item.title}</a>
        </h1>
        <h3>
          <TimeAgo
            className="pub_date"
            date={this.state.item.published_at}
            minPeriod="30"
          />
        </h3>
        <p>{this.state.item.summary}</p>
        <p dangerouslySetInnerHTML={{ __html: this.state.item.content }} />
      </div>
    );
  }

  fetchData() {
    var url = `/api/item/${this.state.item.id}`;
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
      .then(data => {
        this.setState({ item: data });
      })
      .catch(error => store.msgCallback("error", error, "warning"));
  }
}

export default SingleItem;
