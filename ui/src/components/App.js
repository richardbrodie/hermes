import React, { Component } from 'react';
import { Route, Switch, Redirect } from 'react-router-dom';

import Main from './Main'
import Login from './Login'
import store from './store'

import '../styles/App.css'

const PrivateRoute = ({ component: Component, ...rest }) => (
  <Route
    {...rest}
    render={props =>
      store.loggedIn ? (
        <Component {...props} />
      ) : (
          <Redirect
            to={{
              pathname: "/login",
              state: { from: props.location }
            }}
          />
        )
    }
  />
);

class App extends Component {
  constructor(props) {
    super(props)
    const token = localStorage.getItem('token');
    token ? store.setToken(token) : store.removeToken()
  }


  render() {
    return (
      <div>
        <Switch>
          <Route path="/login" component={Login} />
          <PrivateRoute path="/" component={Main} />
        </Switch>
      </div>
    )
  }
}

export default App;
