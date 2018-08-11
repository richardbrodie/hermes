import React, { Component } from 'react';
import { Route, Switch } from 'react-router-dom';
import 'react-notifications-component/dist/theme.css'

import AddFeed from './AddFeed'
import TopBar from './TopBar'
import ItemList from './ItemList'
import FeedList from './FeedList'
import SingleItem from './SingleItem'

import "../styles/Main.css"

export default class Main extends Component {
  render() {
    return (
      <div id="main-view">
        <TopBar />
        <FeedList />

        <Switch>
          <Route path="/feed/:id" component={ItemList} />
          <Route path="/add" component={AddFeed} />
          <Route path="/item" render={(props) => <SingleItem {...props} />} />
        </Switch>
      </div>
    )
  }
}

