import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import TimeAgo from 'react-timeago'

import store from './store'

import "../styles/ItemList.css"

class Items extends Component {
  constructor(props) {
    super(props)
    this.state = { items: [] }
    this.fetchData(props.match.params.id)
  }

  componentDidUpdate(prevProps) {
    let newid = this.props.match.params.id
    if (prevProps.match.params.id !== newid) {
      this.fetchData(newid)
    }
  }

  render() {
    return (
      <div id="feed-items">
        {this.state.items.map((item, i) =>
          < div key={i} className='feed-item' >
            <Link className="title" to={{ pathname: '/item', state: { item: item } }}>{item.title}</Link>
            <TimeAgo className='pub_date' date={item.published_at} minPeriod='30' />
            <div className='desc'>{item.description}</div>
          </div >
        )}
      </div>
    )
  }

  fetchData(id) {
    var url = `http://localhost:4000/items/${id}`;
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
      .then(data => this.setState({ items: data }))
      .catch(error => store.msgCallback('error', error, 'warning'))
  }
}

export default Items;
