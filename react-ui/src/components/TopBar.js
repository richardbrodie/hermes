import React, { Component } from 'react';
import { Link } from 'react-router-dom';

import "../styles/TopBar.css"

class TopBar extends Component {
  render() {
    return (
      <div id="top-bar" >
        <div className="menuMainItem"><Link to='/' >Feeds.rs</Link></div>
        <div className="menuItem"><Link to="/add">Add feed</Link></div>
      </div >
    )
  }
}

export default TopBar
