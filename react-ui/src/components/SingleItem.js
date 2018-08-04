import React, { Component } from 'react';
import TimeAgo from 'react-timeago'

import "../styles/SingleItem.css"

class SingleItem extends Component {
  constructor(props) {
    super(props)
    console.log(props.location.state.item)
    this.state = { item: props.location.state.item }
  }
  render() {
    return (
      <div id="single-item">
        <h1><a href={this.state.item.link}>{this.state.item.title}</a></h1>
        <h3><TimeAgo className='pub_date' date={this.state.item.published_at} minPeriod='30' /></h3>
        <p>{this.state.item.description}</p>
        <p dangerouslySetInnerHTML={{ __html: this.state.item.content }}></p>
      </div>
    )
  }
}

export default SingleItem
