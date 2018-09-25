import React, { PureComponent } from 'react';
import styled from 'styled-components';

class UsernamePasswordForm extends PureComponent {
  render() {
    return (
      <Form>
        <h2>{this.props.heading}</h2>
        <form name={this.props.name} onSubmit={this.props.handler}>
          <Input name="username" type="text" placeholder="username" />
          <Input name="password" type="password" placeholder="password" />
          <Button>{this.props.button}</Button>
        </form>
      </Form>
    );
  }
}

class ChangePasswordForm extends PureComponent {
  render() {
    return (
      <Form>
        <h2>{this.props.heading}</h2>
        <form name={this.props.name} onSubmit={this.props.handler}>
          <Input name="old_password" type="password" placeholder="old password" />
          <Input name="new_password" type="password" placeholder="new password" />
          <Input name="confirm_password" type="password" placeholder="confirm password" />
          <Button>{this.props.button}</Button>
        </form>
      </Form>
    );
  }
}

class AddFeedForm extends PureComponent {
  render() {
    return (
      <Form>
        <h2>{this.props.heading}</h2>
        <form name={this.props.name} onSubmit={this.props.handler}>
          <Input name="feed-url" type="text" placeholder="feed url" />
          <Button>{this.props.button}</Button>
        </form>
      </Form>
    );
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

const OldButton = styled.button`
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

const Button = styled.button`
    border: 1px solid #3498db;
    border-radius: 3px;
    background-color: #3498db;
    color: white;
    display: inline-block;
    text-transform:uppercase;
    font-weight: 700;
    line-height: 22px;
    padding: 0.5em 1em;
    min-width: 5em;
    &:hover {
        background-color: #2a7aaf;
    }
`;

const StyledContainer = styled.div`
  padding-left: 0.5em;
  max-width: 50%;
`;

export { StyledContainer, Input, Button, UsernamePasswordForm, ChangePasswordForm, AddFeedForm }
