import React, { PureComponent } from 'react';
import { Link } from 'react-router-dom';
import TimeAgo from 'react-timeago'; import styled from 'styled-components';

class ItemListItem extends PureComponent {
  render() {
    var item = this.props.item;
    return (
      <Link to={{ pathname: `/item/${item.id}` }}>
        <FeedItem seen={item.seen}>
          <span>{item.title}</span>
          <Description>{item.summary}</Description>
          <StyledTime date={item.published_at} minPeriod='30' />
        </FeedItem>
      </Link>
    )
  }
}

const FeedItem = styled.div`
  font-weight: ${props => props.seen ? '400' : '700'};
  padding: 5px 0 5px 10px;
  border-bottom: 1px solid #e6e5e5;
  display: flex;
    &:hover {
    background: #EDEDED;
  }
`;

const meta = `
  color: #8b8b8b;
  font-size: 0.8em;
  border-left: 1px solid #0a0a0a;
  margin-left: 10px;
  padding-left: 10px;
  padding-right: 5px;
`;

const Description = styled.span`
  ${ meta}
  flex: 1;
  color: #8b8b8b;
  font-size: 0.9em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
`;

const StyledTime = styled(TimeAgo)`
  ${ meta}
  min-width: 85px;
  margin-left: auto;
`;

export { ItemListItem, StyledTime };
