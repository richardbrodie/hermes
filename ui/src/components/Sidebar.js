import React, { Component } from "react"
import { NavLink, Link } from "react-router-dom"
import styled from 'styled-components';

import "../styles/Sidebar.css"

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
              <div data_id={feed.id} className="router-link" key={i}>
                <span className="feed-title">{feed.title}</span>
                <span className="feed-count">{feed.unseen_count}</span>
              </div>
            </NavLink>
          ))}
        </StyledFeedList>
        <div className="btnCtr">
          <Link className="menuItem" to="/add"> Add feed </Link>
          <Link className="menuItem" to="/add"> Settings </Link>
        </div>
      </StyledSidebar>
    )
  }
}

export default Sidebar
