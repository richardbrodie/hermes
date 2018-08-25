import React, { Component } from 'react';
import { Route, Switch } from 'react-router-dom';
import 'react-notifications-component/dist/theme.css'

import AddFeed from './AddFeed'
import ItemList from './ItemList'
import SingleItem from './SingleItem'
import Sidebar from './Sidebar'

import "../styles/Main.css"

export default class Main extends Component {
  render() {
    return (
      <div id="main-view">
        <Sidebar />

        <Switch>
          <Route path="/feed/:id" component={ItemList} />
          <Route path="/add" component={AddFeed} />
          <Route path="/item" render={(props) => <SingleItem {...props} />} />
        </Switch>
      </div>
    )
  }
}

