// theirs
import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router } from 'react-router-dom';

// mine
import App from './components/App';
require('./favicon.png')

ReactDOM.render((
  <Router>
    <App />
  </Router>
), document.getElementById('root'))

