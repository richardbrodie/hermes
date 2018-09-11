import React, { Component } from "react"
import TimeAgo from "react-timeago"

import "../styles/SingleItem.css"

class SingleItem extends Component {
  constructor(props) {
    super(props)
    this.state = { item: props.item }
    props.handler(props.match.params.id)
  }

  shouldComponentUpdate(nextProps, nextState) {
    if (nextProps.item !== this.props.item) {
      this.setState({ item: nextProps.item })
    }
    if (nextState.item !== this.state.item) {
      return true
    }
    return false
  }

  render() {
    if (!this.state.item) {
      return null
    }
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
    )
  }
}

export default SingleItem
