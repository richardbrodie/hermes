import React, { Component } from 'preact';
import { ChangePasswordForm, StyledContainer, UsernamePasswordForm } from './Forms'
import { Button } from './Elements'
// import { Button } from 'rebass'
import * as store from './local_storage';

export default class Settings extends Component {
  constructor(props) {
    super(props);
    this.handlePasswordChange = this.handlePasswordChange.bind(this);
    this.handleAddUser = this.handleAddUser.bind(this);
    this.handleLogout = this.handleLogout.bind(this);
  }

  handlePasswordChange(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    let data = {
      'name': event.target.name,
      'data': {
        'old_password': formData.get('old_password'),
        'new_password': formData.get('new_password'),
        'confirm_password': formData.get('confirm_password')
      }
    };
    this.sendSettings(data)
  }
  handleAddUser(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    let data = {
      'name': event.target.name,
      'data': {
        'username': formData.get('username'),
        'password': formData.get('password'),
      }
    };
    this.sendSettings(data)
  }

  handleLogout(event) {
    event.preventDefault();
    store.remove_jwt();
    this.props.history.push('/');
  }

  render() {
    var adminBlock;
    if (store.token_data().name == "admin") {
      adminBlock = <UsernamePasswordForm name={"add_user"} heading={"Add user"} button={"Save"} handler={this.handleAddUser} />
    }
    return (
      <StyledContainer>
        <h2>User settings</h2>
        <Button onClick={this.handleLogout}>Logout</Button>
        <ChangePasswordForm name={"change_password"} heading={"Change password"} button={"Save"} handler={this.handlePasswordChange} />
        {adminBlock}
      </StyledContainer>
    );
  }
}
