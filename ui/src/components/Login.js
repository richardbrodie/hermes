import React, { Component } from 'react';
import { Redirect } from 'react-router-dom';
import * as store from './local_storage';

import { UsernamePasswordForm, StyledContainer } from './Forms'

class Login extends Component {
  constructor(props) {
    super(props);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    this.login(formData.get('username'), formData.get('password'));
  }

  login(username, password) {
    var url = '/authenticate';
    var body = JSON.stringify({
      username: username,
      password: password
    });
    var headers = new Headers({
      'Content-Type': 'application/json'
    });
    var req = new Request(url, {
      method: 'POST',
      body: body,
      headers: headers
    });
    fetch(req)
      .then(resp => resp.json())
      .then(data => {
        store.set_jwt(data.token);
        this.props.history.push('/');
      })
      .catch(error => {
        store.remove_jwt();
      });
  }

  render() {
    if (store.logged_in()) {
      return <Redirect to="/" />;
    } else {
      return (
        <StyledContainer>
          <UsernamePasswordForm heading={"Please sign in"} button={"login"} handler={this.handleSubmit} />
        </StyledContainer>
      );
    }
  }
}

export default Login;
