import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import TimeAgo from 'react-timeago'
import InfiniteScroll from 'react-infinite-scroller'

import store from './store'

import "../styles/ItemList.css"

class Items extends Component {
  constructor(props) {
    super(props)
    this.state = { items: [], id: props.match.params.id, lastDate: null, hasMore: true }
    this.fetchData = this.fetchData.bind(this);
    this.fetchData()
  }

  componentDidUpdate(prevProps, prevState) {
    if (prevProps.match.params.id !== this.props.match.params.id) {
      this.setState({ items: [], id: this.props.match.params.id, lastDate: null })
    }
    if (prevState.id !== this.state.id) {
      this.fetchData()
    }
  }

  render() {
    var items = []
    this.state.items.map((item, i) =>
      items.push(
        <Link className="title" to={{ pathname: '/item', state: { item: item } }}>
          <div key={i} className={`feed-item seen_${item.seen}`} >
            <span>{item.title}</span>
            <span className='desc meta'>{item.summary}</span>
            <TimeAgo className='pub_date meta' date={item.published_at} minPeriod='30' />
          </div >
        </Link>
      )
    )

    return (
      <div id="feed-items">
        <InfiniteScroll pageStart={0} loadMore={this.fetchData} hasMore={true} initialLoad={false}>
          {items}
        </InfiniteScroll>
      </div>
    )
  }

  fetchData() {
    var url = `/api/items/${this.state.id}`;
    if (this.state.lastDate) {
      url = `${url}?updated=${this.state.lastDate}`
    }
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
        let lastDate = data[data.length - 1].published_at
        if (this.state.lastDate) {
          this.setState((prevState, _props) => ({
            lastDate: lastDate,
            items: prevState.items.concat(data)
          }));
        }
        else {
          this.setState({
            lastDate: lastDate,
            items: data
          });
        }
      })
      .catch(error => store.msgCallback('error', error, 'warning'))
  }
}

export default Items;
