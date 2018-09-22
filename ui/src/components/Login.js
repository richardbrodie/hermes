import React, { Component } from 'react';
import { Redirect } from 'react-router-dom';
import store from './store';
import styled from 'styled-components';

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
        store.setToken(data.token);
        this.props.history.push('/');
      })
      .catch(error => {
        store.removeToken();
      });
  }

  render() {
    if (store.loggedIn) {
      return <Redirect to="/" />;
    } else {
      return (
        <Form>
          <h2 className="form-signin-heading">Please sign in</h2>
          <form className="login-form" onSubmit={this.handleSubmit}>
            <Input name="username" type="text" placeholder="username" />
            <Input name="password" type="password" placeholder="password" />
            <Button>login</Button>
          </form>
        </Form>
      );
    }
  }
}

const Form = styled.div`
  font-family: "Roboto", sans-serif;
  font-size: 14px;
  width: 360px;
  margin: 0 auto 100px;
  position: relative;
  z-index: 1;
  background: #ffffff;
  max-width: 360px;
  padding: 45px;
  text-align: center;
  box-shadow: 0 0 20px 0 rgba(0, 0, 0, 0.2), 0 5px 5px 0 rgba(0, 0, 0, 0.24);
`;

const common_input = `
  outline: 0;
  background: #f2f2f2;
  width: 100%;
  border: 0;
  margin: 0 0 15px;
  padding: 15px;
  box-sizing: border-box;
`;
const Input = styled.input`
  ${ common_input}
`;

const Button = styled.button`
  ${ common_input}
  text-transform: uppercase;
  outline: 0;
  margin: 0;
  background: #4caf50;
  color: #ffffff;
  cursor: pointer;
  &:hover, &:active, &:focus {
    background: #43a047;
  }
`;

export default Login;
