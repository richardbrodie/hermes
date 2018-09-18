import React, { Component } from "react"
import { NavLink, Link } from "react-router-dom"
import styled from 'styled-components';

const StyledSidebar = styled.div`
  border-right: 1px solid #0a0a0a;
  background-color:#EDEDED;
  grid-row: 1 / span 2;
  grid-column: 1;
  display: flex;
  flex-direction: column;
`;

const StyledTopBar = styled.div`
  padding: 1em;
  text-transform: uppercase;
  text-align: center;
  cursor: pointer;
  font-size: 1.3em;
  border-bottom: 1px solid #d1cfcf;
`;

const StyledFeedList = styled.nav`
  grid-row: 2;
  grid-column: 1;
  font-size: 10pt;
  .active {
    font-weight: 600;
  }
`;

const AddFeedButton = styled.div`
  margin-left: auto;
  margin-right: auto;
  margin-top: auto;
`;

const FeedLink = styled.div`
  padding: 10px 10px;
  border-bottom: 1px solid #d1cfcf;
  display: flex;
  &:hover {
    background: rgb(255, 255, 255);
  }
`;

const FeedCount = styled.span`
  margin-left: auto;
  margin-top: auto;
  margin-bottom: auto;
  font-weight: 400;
  border: 1px solid;
  border-radius: 7px;
  background: #2cadee;
  color: white;
  padding: 0 8px;
`;

const MenuLink = styled(Link)`
  text-transform: uppercase;
  cursor: pointer;
  text-align: center;
  padding-left: 10px;
  padding-right: 10px;
  &:hover {
    background: rgb(255, 255, 255);
  }
`;

class Sidebar extends Component {
  render() {
    const feeds = this.props.feeds_data
    return (
      <StyledSidebar>
        <StyledTopBar>
          <Link to="/">hermes</Link>
        </StyledTopBar>
        <StyledFeedList>
          {feeds.map((feed, i) => (
            <NavLink key={i} to={`/feed/${feed.id}`}>
              <FeedLink data_id={feed.id} key={i}>
                <span className="feed-title">{feed.title}</span>
                <FeedCount>{feed.unseen_count}</FeedCount>
              </FeedLink>
            </NavLink>
          ))}
        </StyledFeedList>
        <AddFeedButton>
          <MenuLink to="/add"> Add feed </MenuLink>
        </AddFeedButton>
      </StyledSidebar>
    )
  }
}

export default Sidebar
