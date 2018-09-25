import React, { Component } from 'react';
import { ChangePasswordForm, StyledContainer, UsernamePasswordForm } from './Forms'
import * as store from './local_storage';

export default class Settings extends Component {
  constructor(props) {
    super(props);
    this.handlePasswordChange = this.handlePasswordChange.bind(this);
    this.handleAddUser = this.handleAddUser.bind(this);
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

  sendSettings(data) {
    var url = `/api/settings${store.access_token_str()}`;
    var body = JSON.stringify(data);
    var headers = new Headers({
      'Content-Type': 'application/json',
    });
    var req = new Request(url, {
      method: 'POST',
      body: body,
      headers: headers
    });
    fetch(req)
      .then(resp => {
        if (resp.ok) {
          console.log("send settings success: ", `${resp.status}: ${resp.statusText}`)
        } else {
          console.log("send settings error: ", `${resp.status}: ${resp.statusText}`)
        }
      })
      .catch(error => console.log("add feed error: ", error));
  }

  render() {
    var adminBlock;
    if (store.token_data().name == "admin") {
      adminBlock = <UsernamePasswordForm name={"add_user"} heading={"Add user"} button={"Save"} handler={this.handleAddUser} />
    }
    return (
      <StyledContainer>
        <h2>User settings</h2>
        <ChangePasswordForm name={"change_password"} heading={"Change password"} button={"Save"} handler={this.handlePasswordChange} />
        {adminBlock}
      </StyledContainer>
    );
  }
}
