// import 'jquery'
// import 'popper.js'
// import 'bootstrap'

import './application.scss'

console.log("hello world");

function component() {
  var element = document.createElement('div');

  // Lodash, currently included via a script, is required for this line to work
  element.innerHTML = _.join(['Hello', 'webpack'], ' ');

  return element;
}
