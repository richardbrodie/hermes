import React, { Component } from 'react';

import store from './store'

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
            <span className='title'>{item.title}</span>
            <span className='pub_date'>{item.published_at}</span>
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
