import React, { PureComponent, Component } from "react"
import TimeAgo from "react-timeago"
import styled from 'styled-components';

const StyledItem = styled.div`
  padding-left: 10px;
  grid-row: 1 / span 2;
  grid-column: 2;
  overflow-y: scroll;
`;

class SingleItem extends PureComponent {
  constructor(props) {
    super(props)
    props.handler(props.match.params.id)
  }

  render() {
    var item = this.props.item;
    if (!item) {
      return null
    }
    return (
      <StyledItem>
        <h1>
          <a href={item.link}>{item.title}</a>
        </h1>
        <h3>
          <TimeAgo
            className="pub_date"
            date={item.published_at}
            minPeriod="30"
          />
        </h3>
        <p>{item.summary}</p>
        <p dangerouslySetInnerHTML={{ __html: item.content }} />
      </StyledItem>
    )
  }
}

export default SingleItem
