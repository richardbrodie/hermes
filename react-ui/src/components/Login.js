import React, { Component } from 'react';
import { Redirect } from 'react-router-dom';

import store from './store'
import '../styles/Login.css'

class Login extends Component {
  constructor(props) {
    super(props)
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    this.login(formData.get("username"), formData.get("password"))
  }

  login(username, password) {
    var url = "/authenticate";
    var body = JSON.stringify({
      username: username,
      password: password
    });
    var headers = new Headers({
      "Content-Type": "application/json"
    });
    var req = new Request(url, {
      method: "POST",
      body: body,
      headers: headers
    });
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        store.setToken(data.token)
        this.props.history.push('/')
      }).catch(error => {
        store.removeToken()
      });
  }

  render() {
    if (store.loggedIn) {
      return (<Redirect to="/" />)
    }
    else {
      return (
        <div className="form">
          <h2 className="form-signin-heading">Please sign in</h2>
          <form className="login-form" onSubmit={this.handleSubmit}>
            <input name="username" type="text" placeholder="username" />
            <input name="password" type="password" placeholder="password" />
            <button>login</button>
          </form>
        </div >
      );
    }
  }
}

export default Login;
