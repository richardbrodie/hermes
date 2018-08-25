import React, { Component } from 'react';
import { Route, Switch, Redirect } from 'react-router-dom';
import ReactNotification from "react-notifications-component";

import Main from './Main'
import Login from './Login'
import store from './store'

import '../styles/App.css';

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
    this.addNotification = this.addNotification.bind(this);
    store.setMsgCallback(this.addNotification)
  }

  addNotification(title, message, type) {
    this.notificationDOMRef.addNotification({
      title: title,
      message: message,
      type: type,
      insert: "top",
      container: "top-right",
      animationIn: ["animated", "fadeIn"],
      animationOut: ["animated", "fadeOut"],
      dismiss: { duration: 2000 },
      dismissable: { click: true }
    });
  }

  render() {
    return (
      <div>
        <ReactNotification ref={input => this.notificationDOMRef = input} />
        <Switch>
          <Route path="/login" component={Login} />
          <PrivateRoute path="/" component={Main} />
        </Switch>
      </div>
    )
  }
}

export default App;
