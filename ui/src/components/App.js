import React, { Component } from 'react';
import { Router } from "@reach/router";
// import { injectGlobal } from 'styled-components'

import Main from './Main/Main'
import Login from './Login'
// import * as store from './local_storage'

// injectGlobal`
//   html {
//     text-rendering: optimizeLegibility;
//     font-family: "Open Sans", sans-serif;
//     font-feature-settings: "lnum", "kern";
//     line-height: 1.5em;
//   }
//   body {
//     height: 100vh;
//     margin: 0;
//     padding: 0;
//   }
// `

// const PrivateRoute = ({ component: Component, ...rest }) => (
//   <Route
//     {...rest}
//     render={props =>
//       store.logged_in() ? (
//         <Component {...props} />
//       ) : (
//           <Redirect
//             to={{
//               pathname: "/login",
//               state: { from: props.location }
//             }}
//           />
//         )
//     }
//   />
// );

class App extends Component {
  constructor(props) {
    super(props)
  }


  render() {
    return (
      <div>
        {/* <Route path="/login" component={Login} />
        <PrivateRoute path="/" component={Main} /> */}
        <Route path="/login" component={Main} />
      </div>
    )
  }
}

export default App;
