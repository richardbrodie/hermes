import React, { Component } from 'preact';
import InfiniteScroll from 'react-infinite-scroller';
import styled from 'styled-components';
import { ItemListItem } from './ItemListItem/ItemListItem'

class ItemList extends Component {
  constructor(props) {
    super(props);
    this.props.handler(props.match.params.id)
  }

  shouldComponentUpdate(nextProps) {
    if (nextProps.match.params.id !== this.props.match.params.id) {
      this.props.handler(nextProps.match.params.id)
      return false
    }
    if (nextProps.items_data !== this.props.items_data) {
      return true
    }
    return false
  }

  render() {
    let items_data = this.props.items_data;
    if (!items_data) { return null; }

    var items = [];
    items_data.map((item, i) =>
      items.push(<ItemListItem key={i} item={item} />)
    );

    return (
      <FeedItems>
        <InfiniteScroll
          pageStart={0}
          loadMore={this.props.load_more_handler}
          hasMore={true}
          initialLoad={false}
          useWindow={false}
        >
          {items}
        </InfiniteScroll>
      </FeedItems>
    );
  }
}

const FeedItems = styled.div`
  grid-row: 1 / span 2;
  grid-column: 2;
  overflow-y: scroll;
`;

export default ItemList;
