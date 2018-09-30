import React, { Component } from "preact"
import { Link } from "react-router-dom"
import {
  Logo,
  StyledSidebar,
  StyledTopBar,
  StyledFeedList,
  AddFeedButton,
  FeedLink,
  FeedCount,
  MenuLink
} from './styles'

class Sidebar extends Component {
  render() {
    const feeds = this.props.feeds_data
    return (
      <StyledSidebar>
        <StyledTopBar>
          <Logo src={'/favicon.png'} />
          <Link to="/">hermes</Link>
        </StyledTopBar>
        <StyledFeedList>
          {feeds.map((feed, i) => (
            <FeedLink to={`/feed/${feed.id}`} data_id={feed.id} key={i}>
              <span className="feed-title">{feed.title}</span>
              <FeedCount>{feed.unseen_count}</FeedCount>
            </FeedLink>
          ))}
        </StyledFeedList>
        <AddFeedButton>
          <MenuLink to="/add"> Add feed </MenuLink>
          <MenuLink to="/settings"> Settings </MenuLink>
        </AddFeedButton>
      </StyledSidebar>
    )
  }
}

export default Sidebar
