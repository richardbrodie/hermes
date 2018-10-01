import React from 'react';
import ReactDOM from 'react-dom';
import { Router } from "@reach/router";
import { createGlobalStyle } from 'styled-components'

// mine
import Login from './components/Login';
import Main from './components/Main/Main';
require('./favicon.png')

const GlobalStyle = createGlobalStyle`
  html {
    text-rendering: optimizeLegibility;
    font-family: "Open Sans", sans-serif;
    font-feature-settings: "lnum", "kern";
    line-height: 1.5em;
  }
  body {
    height: 100vh;
    margin: 0;
    padding: 0;
  }
`

ReactDOM.render((
  <div>
    <GlobalStyle />
    <Router>
      <Main path="/*" />
      <Login path="/login" />
    </Router>
  </div>
), document.getElementById('root'))

