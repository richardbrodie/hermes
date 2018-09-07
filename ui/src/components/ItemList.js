import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import TimeAgo from 'react-timeago';
import InfiniteScroll from 'react-infinite-scroller';

import '../styles/ItemList.css';

class ItemList extends Component {
  constructor(props) {
    super(props);
    this.state = { items: props.items_data, id: props.match.params.id };
    // lastDate: null,
    // hasMore: true
    this.props.handler(props.match.params.id)
  }

  shouldComponentUpdate(nextProps, nextState) {
    // if (nextProps.match.params.id === this.state.id) {
    //   // console.log("p/s id")
    // }
    // if (nextProps.match.params.id === this.state.id) {
    //   // console.log("p/s id")
    // }
    if (nextProps.match.params.id !== this.props.match.params.id) {
      // console.log("p/p id")
      this.setState({ id: nextProps.match.params.id })
      this.props.handler(nextProps.match.params.id)
    }
    if (nextProps.items_data !== this.props.items_data) {
      // console.log("p/p items")
      this.setState({ items: nextProps.items_data })
    }
    if (nextState.items_data !== this.state.items) {
      // console.log("s/s items")
    }
    if (nextProps.match.params.id !== this.props.match.params.id) {
      // console.log("s/s id")
    }
    if (nextState.items !== this.state.items) {
      // console.log("rerendering")
      return true
    }
    return false
  }

  render() {
    if (!this.state.items) {
      return null;
    }
    var items = [];
    this.state.items.map((item, i) =>
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
        {items}
        {/* <InfiniteScroll
          pageStart={0}
          loadMore={this.fetchData}
          hasMore={false}
          initialLoad={false}
        >
          {items}
        </InfiniteScroll> */}
      </div>
    );
  }
}

export default ItemList;
