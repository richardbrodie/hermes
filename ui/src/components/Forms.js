import React, { Component } from 'react';
import styled from 'styled-components';
import { Input, Button, Form } from './Elements'

class UsernamePasswordForm extends Component {
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

class ChangePasswordForm extends Component {
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

class AddFeedForm extends Component {
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

const StyledContainer = styled.div`
  padding-left: 0.5em;
  max-width: 50%;
`;

export { StyledContainer, UsernamePasswordForm, ChangePasswordForm, AddFeedForm }
