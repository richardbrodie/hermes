import React, { Component } from 'react';
import { Link } from "@reach/router";
import { FeedItem, Description } from './styles'
import { StyledTime } from '../Elements'

class ItemListItem extends Component {
  render() {
    var item = this.props.item;
    return (
      <Link to={`/item/${item.id}`}>
        <FeedItem seen={item.seen}>
          <span>{item.title}</span>
          <Description>{item.summary}</Description>
          <StyledTime date={item.published_at} minPeriod='30' />
        </FeedItem>
      </Link>
    )
  }
}

export { ItemListItem }
