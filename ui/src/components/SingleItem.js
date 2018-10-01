import React, { Component } from "react"
import styled from 'styled-components';

import { StyledTime } from './Elements';

class SingleItem extends Component {
  constructor(props) {
    super(props)
    props.handler(props.id)
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
          <StyledTime date={item.published_at} minPeriod="30" />
        </h3>
        <p>{item.summary}</p>
        <p dangerouslySetInnerHTML={{ __html: item.content }} />
      </StyledItem>
    )
  }
}

const StyledItem = styled.div`
  padding-left: 10px;
  grid-row: 1 / span 2;
  grid-column: 2;
  overflow-y: scroll;
`;

export default SingleItem
