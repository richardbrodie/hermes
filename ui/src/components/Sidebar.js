import React, { PureComponent } from "react"
import { NavLink, Link } from "react-router-dom"

import "../styles/Sidebar.css"

class Sidebar extends PureComponent {
  render() {
    const feeds = this.props.feeds_data
    return (
      <div id="sidebar">
        <div id="top-bar">
          <Link to="/">hermes</Link>
        </div>
        <nav id="feed-list">
          {feeds.map((feed, i) => (
            <NavLink key={i} to={`/feed/${feed.id}`}>
              <div data_id={feed.id} className="router-link" key={i}>
                <span className="feed-title">{feed.title}</span>
                <span className="feed-count">{feed.unseen_count}</span>
              </div>
            </NavLink>
          ))}
        </nav>
        <div className="btnCtr">
          <Link className="menuItem" to="/add"> Add feed </Link>
          <Link className="menuItem" to="/add"> Settings </Link>
        </div>
      </div>
    )
  }
}

export default Sidebar
