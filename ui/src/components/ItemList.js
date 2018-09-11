import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import TimeAgo from 'react-timeago';
import InfiniteScroll from 'react-infinite-scroller';

import '../styles/ItemList.css';

class ItemList extends Component {
  constructor(props) {
    super(props);
    this.state = {
      items_data: props.items_data,
    }
    this.props.handler(props.match.params.id)
  }

  shouldComponentUpdate(nextProps, nextState) {
    if (nextProps.match.params.id !== this.props.match.params.id) {
      this.props.handler(nextProps.match.params.id)
      return false
    }
    if (nextProps.items_data !== this.props.items_data) {
      this.setState({ items_data: nextProps.items_data })
      return false
    }
    if (nextState.items_data !== this.state.items_data) {
      return true
    }
    return false
  }

  render() {
    if (!this.state.items_data) { return null; }

    var items = [];
    this.state.items_data.map((item, i) =>
      items.push(
        <Link key={i} className="title" to={{ pathname: `/item/${item.id}` }}>
          <div className={`feed-item seen_${item.seen}`}>
            <span>{item.title}</span>
            <span className='desc meta'>{item.summary}</span>
            <TimeAgo className='pub_date meta' date={item.published_at} minPeriod='30' />
          </div >
        </Link>
      )
    );

    return (
      <div id="feed-items">
        <InfiniteScroll
          pageStart={0}
          loadMore={() => {
            console.log("f")
          }}
          hasMore={true}
          initialLoad={false}
          useWindow={false}
        >
          {items}
        </InfiniteScroll>
      </div >
    );
  }
}

export default ItemList;
